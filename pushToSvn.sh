#!/bin/bash

# Removes unwanted files, it's easier than using svn:ignore
rm -rf dist
rm -rf node_modules
rm -rf src-tauri/target

# Add new files
svn add . 
# Commit changes
svn commit -m "Update from GitLab CI" --username $SVN_USERNAME --password $SVN_PASSWORD --non-interactive

# Push changes
svn update --username $SVN_USERNAME --password $SVN_PASSWORD --non-interactive
svn commit -m "Push changes to SVN repository" --username $SVN_USERNAME --password $SVN_PASSWORD --non-interactive
