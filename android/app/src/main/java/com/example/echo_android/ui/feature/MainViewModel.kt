package com.example.echo_android.ui.feature

import android.util.Log
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.echo_android.network.ApolloWrapper
import com.example.echo_android.network.SSEClient
import com.example.echo_android.network.toLce
import com.example.rocketreserver.GetTimelineQuery
import com.example.rocketreserver.type.ReactionTypeGql
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject
import androidx.compose.runtime.getValue

@HiltViewModel
class MainViewModel @Inject constructor(
    private val apolloWrapper: ApolloWrapper
): ViewModel() {
    private val sseClient = SSEClient()
    private val _avatarExpression = MutableStateFlow<String?>(null)
    val avatarExpression: StateFlow<String?> = _avatarExpression.asStateFlow()

    private val _timelineState = MutableStateFlow(ViewState.INITIAL)
    val timelineState = _timelineState.asStateFlow()
    private var isSseConnected = false
    var isLoadingMore by mutableStateOf(false)
        private set

    // SSE接続とtimeline取得
    init {
        fetchTimeline()
    }

    fun onAppResumed() {
        if (!isSseConnected) {
            startRealtimeUpdates()
        }
    }

    fun onAppPaused() {
        sseClient.disconnect()
        isSseConnected = false
    }

    private fun startRealtimeUpdates() {
        viewModelScope.launch {
            try {
                val initialToken = apolloWrapper.generateSseToken()

                if (initialToken != null) {
                    sseClient.startSSE(
                        sseToken = initialToken,
                        onReaction = { reaction ->
                            _avatarExpression.value = reaction.latestReactionForAuthor
                            Log.d("MainViewModel", "アバター表情更新: ${reaction.latestReactionForAuthor}")
                        },
                        tokenProvider = {
                            // 55秒ごとに新しいトークンを提供
                            apolloWrapper.generateSseToken()
                        }
                    )
                }
            } catch (e: Exception) {
                Log.e("MainViewModel", "SSE開始に失敗", e)
            }
        }
    }

    override fun onCleared() {
        super.onCleared()
        sseClient.disconnect()
    }

    fun fetchTimeline() {
        apolloWrapper.fetchTimeline().toLce().onEach { lce ->
            _timelineState.update {
                it.copy(
                    isLoading = lce.isLoading,
                    throwable = lce.getThrowableIfError(),
                    content = lce.getDataIfContent()
                )
            }
        }.launchIn(viewModelScope)
    }

    fun loadMorePosts() {
        if (isLoadingMore) return

        isLoadingMore = true
        viewModelScope.launch {
            try {
                val response = apolloWrapper.fetchTimelineOnce()
                response?.timeline?.let { newPosts ->
                    _timelineState.update { state ->
                        val currentPosts = state.content?.timeline.orEmpty()
                        val existingIds = currentPosts.map { it.id }.toSet()

                        // 重複を除外して追加
                        val uniqueNewPosts = newPosts.filter { it.id !in existingIds }
                        val mergedPosts = currentPosts + uniqueNewPosts

                        state.copy(
                            content = GetTimelineQuery.Data(
                                timeline = mergedPosts
                            )
                        )
                    }
                }
            } catch (e: Exception) {
                Log.e("MainViewModel", "追加読み込みに失敗", e)
            } finally {
                delay(500) // 連続発火を防ぐ
                isLoadingMore = false
            }
        }
    }

    fun toggleReaction(postId: String, reaction: ReactionTypeGql, isActive: Boolean) {
        viewModelScope.launch {
            _timelineState.update { it.copy(isLoading = true, throwable = null) }
            val success = if (isActive) {
                apolloWrapper.removeReaction(postId)
            } else {
                apolloWrapper.addReaction(postId, reaction)
            }

            if (success) {
                _timelineState.update { state ->
                    val updatedReactions = state.userReactions.toMutableMap()
                    if (isActive) {
                        updatedReactions.remove(postId)
                    } else {
                        updatedReactions[postId] = reaction
                    }
                    state.copy(
                        isLoading = false,
                        userReactions = updatedReactions
                    )
                }
            } else {
                _timelineState.update {
                    it.copy(
                        isLoading = false,
                        throwable = Exception("toggleReaction failed")
                    )
                }
            }
        }
    }

    data class ViewState(
        val isLoading: Boolean = false,
        val throwable: Throwable?,
        val content: GetTimelineQuery.Data?,
        val userReactions: Map<String, ReactionTypeGql> = emptyMap(),
    ) {
        companion object {
            val INITIAL = ViewState(
                isLoading = false,
                throwable = null,
                content = null,
                userReactions = emptyMap()
            )
        }
    }
}
