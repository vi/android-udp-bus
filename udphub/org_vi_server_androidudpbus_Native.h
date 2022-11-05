/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class org_vi_server_androidudpbus_Native */

#ifndef _Included_org_vi_server_androidudpbus_Native
#define _Included_org_vi_server_androidudpbus_Native
#ifdef __cplusplus
extern "C" {
#endif
#undef org_vi_server_androidudpbus_Native_STATS_SHORT
#define org_vi_server_androidudpbus_Native_STATS_SHORT 0L
#undef org_vi_server_androidudpbus_Native_STATS_LONG
#define org_vi_server_androidudpbus_Native_STATS_LONG 1L
/*
 * Class:     org_vi_server_androidudpbus_Native
 * Method:    create
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_org_vi_1server_androidudpbus_Native_create
  (JNIEnv *, jclass);

/*
 * Class:     org_vi_server_androidudpbus_Native
 * Method:    configure
 * Signature: (JLjava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_org_vi_1server_androidudpbus_Native_configure
  (JNIEnv *, jclass, jlong, jstring);

/*
 * Class:     org_vi_server_androidudpbus_Native
 * Method:    getError
 * Signature: (J)Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_org_vi_1server_androidudpbus_Native_getError
  (JNIEnv *, jclass, jlong);

/*
 * Class:     org_vi_server_androidudpbus_Native
 * Method:    start
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_org_vi_1server_androidudpbus_Native_start
  (JNIEnv *, jclass, jlong);

/*
 * Class:     org_vi_server_androidudpbus_Native
 * Method:    delete
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_org_vi_1server_androidudpbus_Native_delete
  (JNIEnv *, jclass, jlong);

/*
 * Class:     org_vi_server_androidudpbus_Native
 * Method:    getStats
 * Signature: (JI)Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_org_vi_1server_androidudpbus_Native_getStats
  (JNIEnv *, jclass, jlong, jint);

/*
 * Class:     org_vi_server_androidudpbus_Native
 * Method:    checkConfig
 * Signature: (Ljava/lang/String;)Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_org_vi_1server_androidudpbus_Native_checkConfig
  (JNIEnv *, jclass, jstring);

#ifdef __cplusplus
}
#endif
#endif