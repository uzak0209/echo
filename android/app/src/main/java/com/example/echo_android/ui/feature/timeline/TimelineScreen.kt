package com.example.echo_android.ui.feature.timeline

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.runtime.getValue
import androidx.hilt.navigation.compose.hiltViewModel
import kotlin.text.get

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TimelineScreen(viewModel: TimelineViewModel = hiltViewModel()) {
    val viewState by viewModel.viewState.collectAsState()
    val posts = viewState.content?.timeline

    LaunchedEffect(Unit) {
        viewModel.fetchTimeline()
    }

    PullToRefreshBox(
        isRefreshing = viewState.isLoading,
        onRefresh = { viewModel.fetchTimeline() },
    ) {
        LazyColumn(modifier = Modifier.fillMaxSize()) {
            items(posts ?: emptyList()) { post ->
                PostItem(
                    post = post,
                    userReaction = viewState.userReactions[post.id],
                    onReactionClick = { postId, reactionType, isActive ->
                        viewModel.toggleReaction(postId, reactionType, isActive)
                    }
                )
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


