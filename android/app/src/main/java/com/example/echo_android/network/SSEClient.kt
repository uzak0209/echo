package com.example.echo_android.network

import android.util.Log
import com.launchdarkly.eventsource.ConnectStrategy
import com.launchdarkly.eventsource.EventSource
import com.launchdarkly.eventsource.MessageEvent
import com.launchdarkly.eventsource.background.BackgroundEventHandler
import com.launchdarkly.eventsource.background.BackgroundEventSource
import kotlinx.coroutines.*
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.Json
import java.net.URI
import java.util.concurrent.TimeUnit

@Serializable
data class ReactionEvent(
    @SerialName("post_id") val postId: String,
    @SerialName("reactor_user_id") val reactorUserId: String,
    @SerialName("reaction_type") val reactionType: String,
    @SerialName("timestamp") val timestamp: Long,
    @SerialName("latest_reaction_for_author") val latestReactionForAuthor: String
)

class SSEClient(
    private val baseUrl: String = "http://10.2.0.92:8000"
) {
    private var backgroundEventSource: BackgroundEventSource? = null
    private var reconnectJob: Job? = null
    private var onReactionReceived: ((ReactionEvent) -> Unit)? = null
    private var getNewToken: (suspend () -> String?)? = null

    private val json = Json { ignoreUnknownKeys = true }

    fun startSSE(
        sseToken: String,
        onReaction: (ReactionEvent) -> Unit,
        tokenProvider: suspend () -> String?
    ) {
        onReactionReceived = onReaction
        getNewToken = tokenProvider
        connectSSE(sseToken)
        scheduleReconnect()
    }

    private fun connectSSE(sseToken: String) {
        disconnect()

        val eventHandler = object : BackgroundEventHandler {
            override fun onOpen() {
                Log.d("SSEClient", "SSE接続が確立")
            }

            override fun onClosed() {
                Log.d("SSEClient", "SSE接続終了")
            }

            override fun onMessage(event: String, messageEvent: MessageEvent) {
                try {
                    Log.d("SSEClient", "メッセージ受信: ${messageEvent.data}")
                    val reaction = json.decodeFromString<ReactionEvent>(messageEvent.data)
                    onReactionReceived?.invoke(reaction)
                } catch (e: Exception) {
                    Log.e("SSEClient", "メッセージのパースに失敗", e)
                }
            }

            override fun onComment(comment: String) {
                Log.d("SSEClient", "コメント受信: $comment")
            }

            override fun onError(t: Throwable) {
                Log.e("SSEClient", "SSEエラー", t)
            }
        }

        val uri = URI.create("$baseUrl/api/reactions/events?token=$sseToken")

        backgroundEventSource = BackgroundEventSource.Builder(
            eventHandler,
            EventSource.Builder(
                ConnectStrategy.http(uri)
                    .connectTimeout(30, TimeUnit.SECONDS)
                    .readTimeout(300, TimeUnit.SECONDS)
            )
        ).build()

        backgroundEventSource?.start()
    }

    private fun scheduleReconnect() {
        reconnectJob?.cancel()
        reconnectJob =  CoroutineScope(Dispatchers.IO).launch {
            delay(55000) // 55秒後
            Log.d("SSEClient", "SSE再接続を開始...")

            try {
                val newToken = getNewToken?.invoke()
                if (newToken != null) {
                    connectSSE(newToken)
                    scheduleReconnect() // 次の再接続をスケジュール
                } else {
                    Log.e("SSEClient", "新しいトークンの取得に失敗")
                }
            } catch (e: Exception) {
                Log.e("SSEClient", "再接続に失敗", e)
            }
        }
    }

    fun disconnect() {
        reconnectJob?.cancel()
        reconnectJob = null
        backgroundEventSource?.close()
        backgroundEventSource = null
    }
}
