#!/bin/bash

file="ISSUE.md"

rm -f $file

# Check if project token is provided
if [ -z "$PROJECT_TOKEN" ]; then
	echo "Project token not found."
	exit 1
fi

# Check if URL is provided
if [ -z "$1" ]; then
	echo "URL not provided."
	exit 1
fi

url="$1"

# Make request to fetch issues
response=$(curl -s --header "PRIVATE-TOKEN: $PROJECT_TOKEN" "$url")

# Check if request was successful
if [ $? -ne 0 ]; then
	echo "Failed to fetch issues."
	exit 1
fi

# Check if response is empty
if [ -z "$response" ]; then
	echo "No issues found."
	exit 0
fi

echo $response | jq -c '.[]' | while IFS= read -r obj; do
	id=$(echo $obj | jq -r ".id")
	iid=$(echo $obj | jq -r ".iid")
	title=$(echo $obj | jq -r ".title")
	desc=$(echo $obj | jq -r ".description")
	note_url=$(curl -s --header "PRIVATE-TOKEN: $PROJECT_TOKEN" "$url/$iid/notes")
	echo $note_url | jq -c '.[]' | while IFS= read -r nobj; do
		notes=$(echo $nobj | jq -r ".notes")
	done
	echo "id: $id" >>$file
	echo "iid: $iid" >>$file
	echo "" >>$file
	echo $title >>$file
	echo "" >>$file
	echo $desc >>$file
	echo $notes >>$file
	echo "---------------------------------------------------" >>$file
	echo "" >>$file
	echo "" >>$file
done
