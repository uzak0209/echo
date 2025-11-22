package com.example.echo_android.ui.feature.timeline

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.runtime.getValue
import androidx.compose.runtime.snapshotFlow
import androidx.compose.ui.Alignment
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import com.example.echo_android.ui.feature.MainViewModel
import kotlin.compareTo
import kotlin.text.compareTo
import kotlin.text.get

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TimelineScreen(viewModel: MainViewModel = hiltViewModel()) {
    val viewState by viewModel.timelineState.collectAsState()
    val posts = viewState.content?.timeline ?: emptyList()
    val listState = rememberLazyListState()
    val isLoadingMore = viewModel.isLoadingMore

    LaunchedEffect(listState, isLoadingMore) {
        snapshotFlow {
            val layoutInfo = listState.layoutInfo
            val lastVisibleItem = layoutInfo.visibleItemsInfo.lastOrNull()
            val totalItems = layoutInfo.totalItemsCount

            // 最後から3つ以内のアイテムが見えたら読み込み
            lastVisibleItem != null &&
                    lastVisibleItem.index >= totalItems - 3 &&
                    totalItems > 0 &&
                    !isLoadingMore
        }.collect { shouldLoad ->
            if (shouldLoad) {
                viewModel.loadMorePosts()
            }
        }
    }

    PullToRefreshBox(
        isRefreshing = viewState.isLoading,
        onRefresh = { viewModel.fetchTimeline() },
    ) {
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            state = listState
        ) {
            items(posts, key = { it.id }) { post ->
                PostItem(
                    post = post,
                    userReaction = viewState.userReactions[post.id],
                    onReactionClick = { postId, reactionType, isActive ->
                        viewModel.toggleReaction(postId, reactionType, isActive)
                    }
                )
            }

            // 読み込み中インジケーター
            if (isLoadingMore && posts.isNotEmpty()) {
                item {
                    CircularProgressIndicator(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(16.dp)
                            .wrapContentWidth(Alignment.CenterHorizontally)
                    )
                }
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun TimelineScreenPreview() {
    MaterialTheme {
        TimelineScreen()
    }
}


