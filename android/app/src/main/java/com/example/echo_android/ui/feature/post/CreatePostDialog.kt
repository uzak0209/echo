package com.example.echo_android.ui.feature.post

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import androidx.compose.ui.tooling.preview.Preview
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
                enabled = !postState.isLoading && content.isNotBlank()
            ) {
                if (postState.isLoading) {
                    CircularProgressIndicator()
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
        title = { Text("新規投稿") },
        text = {
            Column {
                OutlinedTextField(
                    value = content,
                    onValueChange = { content = it },
                    placeholder = { Text("投稿内容を入力") },
                    modifier = Modifier.fillMaxWidth(),
                    minLines = 3,
                    enabled = !postState.isLoading
                )
                if (postState.error != null) {
                    Text(
                        text = postState.error!!,
                        color = MaterialTheme.colorScheme.error
                    )
                }
            }
        },
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
