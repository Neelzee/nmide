#!/bin/bash

# Add new files
svn add .

# Commit changes
svn commit -m "Update from GitLab CI" --username $SVN_USERNAME --password $SVN_PASSWORD --non-interactive

# Push changes
svn update --username $SVN_USERNAME --password $SVN_PASSWORD --non-interactive
svn commit -m "Push changes to SVN repository" --username $SVN_USERNAME --password $SVN_PASSWORD --non-interactive
