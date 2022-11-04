package org.vi_server.androidudpbus;

public class Native {
    public static final int STATS_SHORT = 0;
    public static final int STATS_LONG = 1;

    public long self;

    public static long create() {
        return 1;
    }
    public static void configure(long instance, String config) {

    }
    public static String getError(long instance) {
        return null;
    }
    public static void start(long instance) {

    }
    public static void delete(long instance) {

    }
    public static String getStats(long instance, int type) {
        return "Stats";
    }
    public static String checkConfig(String config) {
        if (!config.startsWith("[")) {
            return "Not starts with [";
        }
        return null;
    }
}
