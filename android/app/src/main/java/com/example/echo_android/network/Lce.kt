package com.example.echo_android.network

import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.onStart

/**
 * LCE (Loading, Content, Error) パターン
 * 非同期操作の状態を表現するためのsealed class
 */
sealed class Lce<out T> {
    /**
     * ローディング中の状態
     */
    data object Loading : Lce<Nothing>()

    /**
     * コンテンツを取得できた状態
     */
    data class Content<T>(val data: T) : Lce<T>()

    /**
     * エラーが発生した状態
     */
    data class Error(val throwable: Throwable) : Lce<Nothing>()

    /**
     * ローディング中かどうか
     */
    val isLoading: Boolean
        get() = this is Loading

    /**
     * エラーの場合、Throwableを返す。それ以外はnull
     */
    fun getThrowableIfError(): Throwable? = when (this) {
        is Error -> throwable
        else -> null
    }

    /**
     * コンテンツの場合、データを返す。それ以外はnull
     */
    fun getDataIfContent(): T? = when (this) {
        is Content -> data
        else -> null
    }
}

/**
 * Flow<T>をFlow<Lce<T>>に変換する拡張関数
 */
fun <T> Flow<T>.toLce(): Flow<Lce<T>> = this
    .map<T, Lce<T>> { Lce.Content(it) }
    .onStart { emit(Lce.Loading) }
    .catch { emit(Lce.Error(it)) }
