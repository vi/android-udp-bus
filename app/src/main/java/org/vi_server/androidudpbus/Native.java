package org.vi_server.androidudpbus;

public class Native {
    static {
        System.loadLibrary("udphub");
    }

    public static final int STATS_SHORT = 0;
    public static final int STATS_LONG = 1;

    public long self;

    public static native long create();
    public static native void start(long instance, String config);
    public static native String getError(long instance) ;
    public static native void delete(long instance);
    public static native String getStats(long instance, int type);
    public static native String checkConfig(String config);
}
