package cpzkp

/*
#include <stdlib.h>
#include "cpzkp.h"
*/
import "C"
import (
	"errors"
	"unsafe"
)

// Group represents a cryptographic group
type Group struct {
	ptr unsafe.Pointer
}

// Point represents a point on the curve
type Point struct {
	ptr unsafe.Pointer
}

// Proof represents a zero-knowledge proof
type Proof struct {
	ptr unsafe.Pointer
}

// NewGroup creates a new cryptographic group
func NewGroup() (*Group, error) {
	ptr := C.cpzkp_group_new()
	if ptr == nil {
		return nil, errors.New("failed to create group")
	}
	return &Group{ptr: ptr}, nil
}

// Free releases the group resources
func (g *Group) Free() {
	if g.ptr != nil {
		C.cpzkp_group_free(g.ptr)
		g.ptr = nil
	}
}

// GenerateKey generates a new key pair
func (g *Group) GenerateKey() (*Point, *Point, error) {
	var publicKey, privateKey unsafe.Pointer
	if C.cpzkp_generate_key(g.ptr, &publicKey, &privateKey) != 0 {
		return nil, nil, errors.New("failed to generate key")
	}
	return &Point{ptr: publicKey}, &Point{ptr: privateKey}, nil
}

// CreateProof creates a zero-knowledge proof
func (g *Group) CreateProof(privateKey *Point) (*Proof, error) {
	proof := C.cpzkp_create_proof(g.ptr, privateKey.ptr)
	if proof == nil {
		return nil, errors.New("failed to create proof")
	}
	return &Proof{ptr: proof}, nil
}

// VerifyProof verifies a zero-knowledge proof
func (g *Group) VerifyProof(publicKey *Point, proof *Proof) (bool, error) {
	result := C.cpzkp_verify_proof(g.ptr, publicKey.ptr, proof.ptr)
	if result < 0 {
		return false, errors.New("verification failed")
	}
	return result == 1, nil
}

// SerializePoint serializes a point to bytes
func (p *Point) Serialize() ([]byte, error) {
	var length C.size_t
	data := C.cpzkp_point_serialize(p.ptr, &length)
	if data == nil {
		return nil, errors.New("failed to serialize point")
	}
	defer C.free(unsafe.Pointer(data))
	return C.GoBytes(unsafe.Pointer(data), C.int(length)), nil
}

// DeserializePoint deserializes a point from bytes
func DeserializePoint(data []byte) (*Point, error) {
	ptr := C.cpzkp_point_deserialize((*C.uint8_t)(unsafe.Pointer(&data[0])), C.size_t(len(data)))
	if ptr == nil {
		return nil, errors.New("failed to deserialize point")
	}
	return &Point{ptr: ptr}, nil
}

// Free releases the point resources
func (p *Point) Free() {
	if p.ptr != nil {
		C.cpzkp_point_free(p.ptr)
		p.ptr = nil
	}
}

// Free releases the proof resources
func (p *Proof) Free() {
	if p.ptr != nil {
		C.cpzkp_proof_free(p.ptr)
		p.ptr = nil
	}
}
