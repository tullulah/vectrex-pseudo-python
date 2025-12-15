# Git Integration Testing Guide

## Quick Start

The git integration is fully implemented and ready for testing. Here's how to use it:

## Prerequisites

1. **Git Repository**: Your VPy project must be a git repository
   ```bash
   cd /path/to/vpy/project
   git init  # If not already a git repo
   ```

2. **Initial Commit**: Make at least one commit
   ```bash
   git add .
   git commit -m "Initial commit"
   ```

3. **IDE Running**: Start the IDE with git integration
   ```bash
   npm run dev  # From project root
   ```

## Testing Workflow

### 1. View Git Status

1. Click the **Git** icon in the Activity Bar (sidebar)
2. You should see:
   - **Staged Changes** section (green, files ready to commit)
   - **Changes** section (yellow, files not staged)
   - Status icons (M=Modified, A=Added, D=Deleted, ?=Untracked)

### 2. Stage a File

1. Modify a `.vpy` file in the project
2. In the **Changes** section, click the **+** button next to the file
3. Expected result: File moves to **Staged Changes** with green indicator

### 3. Unstage a File

1. In the **Staged Changes** section, click the **−** button
2. Expected result: File returns to **Changes** section

### 4. Create a Commit

1. Stage one or more files (use + button)
2. Type a commit message in the text area (e.g., "Add new feature")
3. Either:
   - Click the **Commit** button, OR
   - Press **Ctrl+Enter** (Cmd+Enter on macOS)
4. Expected result:
   - Message clears automatically
   - Files disappear from panel (committed)
   - `git log` shows your new commit

### 5. Verify Commit

Open terminal and verify:
```bash
git log --oneline  # Should show your new commit
git status         # Should show "nothing to commit"
```

## Test Scenarios

### Scenario A: Multiple Files
1. Modify 3 different `.vpy` files
2. Stage only 2 of them
3. Verify: Git panel shows 2 staged, 1 unstaged
4. Commit: Verify only staged files are committed

### Scenario B: Untracked Files
1. Create a new `.vpy` file in the project
2. Don't add it to git yet
3. Expected: Git panel shows it with **?** status (untracked)
4. Stage it: Click + button
5. Commit: Include new file in commit

### Scenario C: Delete and Stage
1. Delete a `.vpy` file
2. Stage the deletion: Click + button next to deleted file
3. Expected: Shows **D** (deleted) status
4. Commit: File deletion is recorded

## Troubleshooting

### "No changes found"
- The project folder is not a git repository
- Solution: Run `git init` in project directory

### Git panel shows no changes but files were modified
- Git might not have detected the change yet
- Solution: Save the file and wait 1 second

### "Failed to stage file" error
- Git repository might be corrupted
- Solution: Check `git status` in terminal

### Commit button is disabled
- You need to stage files first (use + button)
- Solution: Ensure at least one file is in "Staged Changes"

## Implementation Details

### Backend Architecture
- **Handler**: `git:status` - Gets repository status
- **Handler**: `git:stage` - Stages files for commit
- **Handler**: `git:unstage` - Removes files from staging
- **Handler**: `git:commit` - Creates commit with message
- **Library**: simple-git (node.js git wrapper)

### Frontend Components
- **GitPanel.tsx**: Main UI component
- **GitPanel.css**: Styling with status colors
- **Activity Bar**: Toggle between Files and Git views

### IPC Bridge
- **Preload Script**: Exposes `window.git` API
- **Main Process**: Handles git operations via simple-git

## Performance Notes

- Status checking is fast (<100ms for typical projects)
- Large repositories may take longer
- Auto-refresh happens after each stage/unstage/commit
- No manual refresh needed

## Advanced Features (Future)

These can be added in future updates:
- **Push/Pull**: Sync with remote repositories
- **Branches**: Create, switch, and merge branches
- **History**: View commit log with filtering
- **Diff**: See file changes before committing
- **Stash**: Temporarily save changes
- **Tags**: Create version tags

---

**Status**: ✅ Ready for Testing
**Last Updated**: 2025-12-10
**Branch**: feature/git-integration

