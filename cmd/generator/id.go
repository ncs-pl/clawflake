// Clawflake ID number generation.

package main

import (
	"errors"
	"flag"
	"sync"
	"time"

	"go.uber.org/zap"
)

var (
	epoch     *uint = flag.Uint("epoch", 0, "The epoch to use to generate ID numbers.")
	machineId *uint = flag.Uint("machine_id", 255, "The ID of the current machine worker. Should be between 0 and 127.")
)

// IdGenerator generates Clawflake ID numbers
type IdGenerator struct {
	// Sequence number, varies between 0 and 4096
	seq       uint16
	last_time int64
	mu        sync.Mutex
	logger    *zap.Logger
}

// GetTime returns the current time based on the configured epoch
func (i *IdGenerator) GetTime() int64 {
	return time.Now().UTC().UnixMilli() - int64(*epoch)
}

// NextId generates a Clawflake ID number.
func (i *IdGenerator) NextId() (uint64, error) {
	if !flag.Parsed() {
		return 0, errors.New("unparsed flags")
	}

	// Generation:

	var id uint64
	i.mu.Lock()
	defer i.mu.Unlock()

	t := i.GetTime()
	l := i.logger.With(zap.Int64("last_time", i.last_time),
		zap.Uint16("seq", i.seq), zap.Uint("epoch", *epoch))

	if t < i.last_time {
		// Time went backward.
		l.Warn("time went backward, sleeping 10ms", zap.Int64("current_time", t))
		time.Sleep(10 * time.Millisecond)

		t = i.GetTime()
	}

	if t == i.last_time {
		// If the generation is in the same millisecond, increment the sequence
		// number if it is possible.
		if i.seq == 4096 {
			i.seq = 0
			l.Debug("rolling over sequence number", zap.Uint16("seq", 0))
		} else {
			i.seq += 1
			l.Debug("incrementing sequence number", zap.Uint16("seq", i.seq))
		}
	} else {
		// Reset the sequence number each millisecond.
		i.seq = 0
		l.Debug("resetting sequence number", zap.Uint16("seq", 0))
	}

	i.last_time = t
	id = uint64(t)<<19 | uint64(*machineId)<<7 | uint64(i.seq)
	l.Info("generated id number", zap.Uint64("generated_id", id))

	return id, nil
}

// NewIdGenerator creates a new IdGenerator
func NewIdGenerator(l *zap.Logger) *IdGenerator {
	return &IdGenerator{
		seq:       0,
		last_time: 0,
		mu:        sync.Mutex{},
		logger:    l,
	}
}
