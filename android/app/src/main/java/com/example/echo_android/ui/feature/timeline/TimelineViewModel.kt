package com.example.echo_android.ui.feature.timeline

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.room.util.copy
import com.example.echo_android.network.ApolloWrapper
import com.example.echo_android.network.Lce
import com.example.echo_android.network.toLce
import com.example.rocketreserver.GetTimelineQuery
import com.example.rocketreserver.type.ReactionTypeGql
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class TimelineViewModel @Inject constructor(
    private val apolloWrapper: ApolloWrapper
) : ViewModel() {
    private val _viewState = MutableStateFlow(ViewState.INITIAL)
    val viewState = _viewState.asStateFlow()
    fun fetchTimeline() {
        apolloWrapper.fetchTimeline().toLce().onEach { lce ->
            _viewState.update {
                it.copy(
                    isLoading = lce.isLoading,
                    throwable = lce.getThrowableIfError(),
                    content = lce.getDataIfContent()
                )
            }
        }.launchIn(viewModelScope)
    }
    fun toggleReaction(postId: String, reaction: ReactionTypeGql, isActive: Boolean) {
        viewModelScope.launch {
            _viewState.update { it.copy(isLoading = true, throwable = null) }
            val success = if (isActive) {
                apolloWrapper.removeReaction(postId)
            } else {
                apolloWrapper.addReaction(postId, reaction)
            }

            if (success) {
                _viewState.update { state ->
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
                _viewState.update {
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
        val userReactions: Map<String, ReactionTypeGql> = emptyMap() // postId -> reactionType
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