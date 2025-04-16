package com.cpzkp;

public class CPZKP {
    static {
        System.loadLibrary("cpzkp");
    }

    private long groupPtr;

    public CPZKP() {
        this.groupPtr = createGroup();
        if (this.groupPtr == 0) {
            throw new RuntimeException("Failed to create group");
        }
    }

    @Override
    protected void finalize() throws Throwable {
        try {
            if (this.groupPtr != 0) {
                freeGroup(this.groupPtr);
                this.groupPtr = 0;
            }
        } finally {
            super.finalize();
        }
    }

    public KeyPair generateKey() {
        long[] keys = new long[2];
        if (generateKey(this.groupPtr, keys) != 0) {
            throw new RuntimeException("Failed to generate key");
        }
        return new KeyPair(new Point(keys[0]), new Point(keys[1]));
    }

    public Proof createProof(Point privateKey) {
        long proofPtr = createProof(this.groupPtr, privateKey.getPtr());
        if (proofPtr == 0) {
            throw new RuntimeException("Failed to create proof");
        }
        return new Proof(proofPtr);
    }

    public boolean verifyProof(Point publicKey, Proof proof) {
        int result = verifyProof(this.groupPtr, publicKey.getPtr(), proof.getPtr());
        if (result < 0) {
            throw new RuntimeException("Verification failed");
        }
        return result == 1;
    }

    // Native method declarations
    private native long createGroup();
    private native void freeGroup(long groupPtr);
    private native int generateKey(long groupPtr, long[] keys);
    private native long createProof(long groupPtr, long privateKeyPtr);
    private native int verifyProof(long groupPtr, long publicKeyPtr, long proofPtr);

    public static class Point {
        private long ptr;

        public Point(long ptr) {
            this.ptr = ptr;
        }

        public long getPtr() {
            return ptr;
        }

        public byte[] serialize() {
            byte[] data = serializePoint(this.ptr);
            if (data == null) {
                throw new RuntimeException("Failed to serialize point");
            }
            return data;
        }

        public static Point deserialize(byte[] data) {
            long ptr = deserializePoint(data);
            if (ptr == 0) {
                throw new RuntimeException("Failed to deserialize point");
            }
            return new Point(ptr);
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                if (this.ptr != 0) {
                    freePoint(this.ptr);
                    this.ptr = 0;
                }
            } finally {
                super.finalize();
            }
        }

        // Native method declarations
        private native byte[] serializePoint(long pointPtr);
        private native static long deserializePoint(byte[] data);
        private native void freePoint(long pointPtr);
    }

    public static class Proof {
        private long ptr;

        public Proof(long ptr) {
            this.ptr = ptr;
        }

        public long getPtr() {
            return ptr;
        }

        @Override
        protected void finalize() throws Throwable {
            try {
                if (this.ptr != 0) {
                    freeProof(this.ptr);
                    this.ptr = 0;
                }
            } finally {
                super.finalize();
            }
        }

        // Native method declarations
        private native void freeProof(long proofPtr);
    }

    public static class KeyPair {
        private final Point publicKey;
        private final Point privateKey;

        public KeyPair(Point publicKey, Point privateKey) {
            this.publicKey = publicKey;
            this.privateKey = privateKey;
        }

        public Point getPublicKey() {
            return publicKey;
        }

        public Point getPrivateKey() {
            return privateKey;
        }
    }
} 