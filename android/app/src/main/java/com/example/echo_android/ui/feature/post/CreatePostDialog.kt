package com.example.echo_android.ui.feature.post

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Create
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel

@Composable
fun CreatePostDialog(
    onDismiss: () -> Unit,
    viewModel: CreatePostViewModel = hiltViewModel()
) {
    // ダイアログ表示時に状態をリセット
    LaunchedEffect(Unit) {
        viewModel.resetState()
    }

    // TODO: imageUrlに対応
    var content by remember { mutableStateOf("") }
    val postState by viewModel.state.collectAsState()

    // 投稿成功時にダイアログを閉じる
    LaunchedEffect(postState.posted) {
        if (postState.posted) {
            onDismiss()
        }
    }

    AlertDialog(
        onDismissRequest = onDismiss,
        confirmButton = {
            Button(
                onClick = {
                    viewModel.createPost(content, null)
                },
                enabled = !postState.isLoading && content.isNotBlank() && content.length <= 500,
                modifier = Modifier.height(40.dp)
            ) {
                if (postState.isLoading) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(20.dp),
                        color = MaterialTheme.colorScheme.onPrimary,
                        strokeWidth = 2.dp
                    )
                } else {
                    Text("投稿")
                }
            }
        },
        dismissButton = {
            TextButton(
                onClick = onDismiss,
                enabled = !postState.isLoading
            ) {
                Text("キャンセル")
            }
        },
        icon = {
            Icon(
                imageVector = Icons.Default.Create,
                contentDescription = "Create",
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
        },
        title = {
            Text(
                text = "新規投稿",
                style = MaterialTheme.typography.headlineSmall
            )
        },
        text = {
            Column(
                modifier = Modifier.fillMaxWidth(),
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                OutlinedTextField(
                    value = content,
                    onValueChange = { content = it },
                    label = { Text("投稿内容") },
                    placeholder = { Text("何を共有しますか?") },
                    modifier = Modifier.fillMaxWidth(),
                    minLines = 4,
                    maxLines = 8,
                    enabled = !postState.isLoading,
                    supportingText = {
                        Text(
                            text = "${content.length} / 500文字",
                            style = MaterialTheme.typography.bodySmall,
                            color = if (content.length > 500) {
                                MaterialTheme.colorScheme.error
                            } else {
                                MaterialTheme.colorScheme.onSurfaceVariant
                            }
                        )
                    },
                    isError = content.length > 500
                )
                if (postState.error != null) {
                    Text(
                        text = postState.error!!,
                        color = MaterialTheme.colorScheme.error,
                        style = MaterialTheme.typography.bodySmall
                    )
                }
            }
        }
    )
}

@Preview(showBackground = true)
@Composable
fun CreatePostDialogPreview() {
    MaterialTheme {
        CreatePostDialog(
            onDismiss = {}
        )
    }
}
