package org.vi_server.androidudpbus;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.app.Service;
import android.content.Context;
import android.content.Intent;
import android.os.Build;
import android.os.IBinder;
import android.os.PowerManager;
import android.widget.Toast;

import java.util.Timer;
import java.util.TimerTask;

public class Serv extends Service {
    private static final String CHANNEL_DEFAULT_IMPORTANCE = "default";
    private static final int ONGOING_NOTIFICATION_ID = 1;
    private static Timer timer;
    private static Notification.Builder nb;
    private static PowerManager.WakeLock wl;

    public static Native instance = new Native();
    public static String startupError = null;

    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }

    @Override
    public void onCreate() {
        super.onCreate();
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        String config = intent.getStringExtra("config");
        synchronized (instance) {
            if (instance.self != 0) {
                Toast toast = Toast.makeText(this, "UDPbus already running", Toast.LENGTH_SHORT);
                toast.show();
                return super.onStartCommand(intent, flags, startId);
            }

            instance.self = Native.create();
        }
        startupError = null;

        Intent notificationIntent = new Intent(this, MainActivity.class);
        PendingIntent pendingIntent =
                PendingIntent.getActivity(this, 0, notificationIntent,
                        PendingIntent.FLAG_IMMUTABLE);


        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            int importance = NotificationManager.IMPORTANCE_LOW;
            NotificationChannel channel = new NotificationChannel(CHANNEL_DEFAULT_IMPORTANCE, "UDPbus", importance);
            channel.setDescription("UDPbus running");
            NotificationManager notificationManager = getSystemService(NotificationManager.class);
            notificationManager.createNotificationChannel(channel);
        }

        CharSequence notiftext = getText(R.string.service_desc);

        boolean failed = false;

        synchronized (instance) {
            Native.start(instance.self, config);
            startupError = Native.getError(instance.self);
            if (startupError != null) {
                failed = true;
            }

            if (failed) {
                Native.delete(instance.self);
                instance.self = 0;
                this.stopForeground(true);
                return super.onStartCommand(intent, flags, startId);
            }
        }

        nb = new Notification.Builder(this)
                    .setContentTitle(getText(R.string.app_name))
                    .setContentText(notiftext)
                    .setSmallIcon(R.drawable.udp)
                    .setContentIntent(pendingIntent);

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            nb.setChannelId(CHANNEL_DEFAULT_IMPORTANCE);
        }
        Notification notification = nb.build();

        startForeground(ONGOING_NOTIFICATION_ID, notification);

        PowerManager pm = (PowerManager)this.getSystemService(
                Context.POWER_SERVICE);
        wl = pm.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "UDPbus:wl");
        wl.acquire();

        timer = new Timer();
        timer.scheduleAtFixedRate(new TimerTask() {
            @Override
            public void run() {
                String stats = "?";
                synchronized (instance) {
                    if (instance.self != 0) {
                        stats = Native.getStats(instance.self, Native.STATS_SHORT);
                    }
                }
                nb.setContentText(stats);
                Notification notification = nb.build();
                NotificationManager notificationManager = (NotificationManager) getSystemService(NOTIFICATION_SERVICE);
                notificationManager.notify(ONGOING_NOTIFICATION_ID, notification);
            }
        }, 0, 1000);

        return super.onStartCommand(intent, flags, startId);
    }

    @Override
    public void onDestroy() {
        timer.cancel();
        timer = null;
        synchronized (instance) {
            if (instance.self != 0) {
                Native.delete(instance.self);
                instance.self = 0;
            }
        }
        if (wl != null) {
            wl.release();
        }
        super.onDestroy();
    }
}
