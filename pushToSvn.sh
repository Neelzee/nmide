#!/bin/bash

# Removes unwanted files, it's easier than using svn:ignore
rm -rf dist
rm -rf node_modules
rm -rf src-tauri/target

# Push changes
svn commit -m "Push changes to SVN repository" --username $SVN_USERNAME --password $SVN_PASSWORD --non-interactive
