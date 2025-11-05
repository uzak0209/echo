package com.echo.app.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.echo.app.ui.components.CreatePostCard
import com.echo.app.ui.components.TimelineList
import com.echo.app.ui.viewmodel.EchoViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun EchoApp(
    viewModel: EchoViewModel = viewModel()
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Echo") }
            )
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            Text(
                text = "A validation-free social network",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )

            CreatePostCard(
                onPostCreated = { content ->
                    viewModel.createPost(content)
                }
            )

            TimelineList(
                posts = viewModel.posts,
                isLoading = viewModel.isLoading,
                onRefresh = { viewModel.loadTimeline() }
            )
        }
    }

    LaunchedEffect(Unit) {
        viewModel.loadTimeline()
    }
}
