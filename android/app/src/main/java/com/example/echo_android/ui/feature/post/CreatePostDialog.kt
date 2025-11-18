package com.example.echo_android.ui.feature.post

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
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
    // TODO: imageUrlに対応
    var content by remember { mutableStateOf("") }
    val posted by viewModel.state.collectAsState()

    if (posted) {
        onDismiss()
    }

    AlertDialog(
        onDismissRequest = onDismiss,
        confirmButton = {
            Button(onClick = {
                viewModel.createPost(content, "")
            }) {
                Text("投稿")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("キャンセル")
            }
        },
        title = { Text("新規投稿") },
        text = {
            Column {
                OutlinedTextField(
                    value = content,
                    onValueChange = { content = it },
                    placeholder = { },
                    modifier = Modifier.fillMaxWidth(),
                    minLines = 3
                )
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
