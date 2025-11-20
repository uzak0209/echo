package com.example.echo_android.ui.feature

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Create
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Tab
import androidx.compose.material3.TabRow
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.example.echo_android.ui.feature.avatar.AvatarScreen
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import com.example.echo_android.ui.feature.post.CreatePostDialog
import com.example.echo_android.ui.feature.timeline.TimelineScreen

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen() {
    var selectedTabIndex by remember { mutableStateOf(0) }
    val tabs = listOf("Timeline", "Avatar")

    var showDialog by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TabRow(selectedTabIndex = selectedTabIndex) {
                tabs.forEachIndexed { index, title ->
                    Tab(
                        selected = selectedTabIndex == index,
                        onClick = { selectedTabIndex = index },
                        text = { Text(title) }
                    )
                }
            }
        },
        floatingActionButton = {
            if (selectedTabIndex == 0) FloatingActionButton(
                onClick = { showDialog = true }
            ) {
                Icon(
                    imageVector = Icons.Default.Create,
                    contentDescription = "Create Post"
                )
            }
        }
    ) { innerPadding ->
        Box(
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxSize(),
            contentAlignment = Alignment.Center
        ) {
            when (selectedTabIndex) {
                0 -> TimelineScreen()
                1 -> AvatarScreen()
            }
        }

        if (showDialog) {
            CreatePostDialog(
                onDismiss = { showDialog = false }
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
fun MainScreenPreview() {
    MainScreen()
}