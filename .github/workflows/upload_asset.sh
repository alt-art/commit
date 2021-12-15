#!/usr/bin/env bash
#
# Author: Christian Duerr
# License: Apache-2.0
# https://github.com/alacritty/alacritty/blob/master/.github/workflows/upload_asset.sh
#
#
# This script accepts the following parameters:
#
# * "owner/repo"
# * file_path
# * token
#
# Script to upload a release asset
#
# Example:
#
# upload_asset.sh "alt-art/lyrs" ./build.zip $GITHUB_TOKEN

repo=$1
file_path=$2
file_name=${file_path##*/}
token=$3

echo "Starting asset upload from $file_name to $repo."
# Define variables.
TAG="$(git describe --tags --abbrev=0)"
if [ -z "$TAG" ]; then
    printf "\e[31mError: Unable to find git tag\e[0m\n"
    exit 1
fi
echo "Git tag: $TAG"
GH_API="https://api.github.com"
GH_REPO="$GH_API/repos/$repo"
AUTH="Authorization: Bearer $token"

# Validate token.
curl -o /dev/null -sH "$AUTH" "$GH_REPO" || { echo "Error: Invalid repo, token or network issue!";  exit 1; }

echo "Checking for existing release..."
upload_url=$(\
    curl \
        -H "$AUTH" \
        "$GH_REPO/releases" \
        2> /dev/null \
    | grep -E "(upload_url|tag_name)" \
    | paste - - \
    | grep -e "tag_name\": \"$TAG\"" \
    | head -n 1 \
    | sed 's/.*\(https.*assets\).*/\1/' \
)

# Create a new release if we didn't find one for this tag.
if [ -z "$upload_url" ]; then
    echo "No release found."
    echo "Creating new release..."

    # Create new release.
    response=$(
        curl -f \
            -X POST \
            -H "$AUTH" \
            -d "{\"tag_name\":\"$TAG\",\"draft\":true}" \
            "$GH_REPO/releases" \
            2> /dev/null\
    )

    # Abort if the release could not be created.
    if [ $? -ne 0 ]; then
        printf "\e[31mError: Unable to create new release.\e[0m\n"
        exit 1;
    fi

    # Extract upload URL from new release.
    upload_url=$(\
        echo "$response" \
        | grep "upload_url" \
        | sed 's/.*: "\(.*\){.*/\1/' \
    )
fi

if [ -z "$upload_url" ]; then
    printf "\e[31mError: Unable to find release upload url.\e[0m\n"
    exit 2
fi

# Upload asset

echo "Uploading asset $file_name to $TAG..."
curl -f \
    -X POST \
    -H "$AUTH" \
    -H "Content-Type: application/octet-stream" \
    --data-binary @"$file_path" \
    "$upload_url?name=$file_name" \
    &> /dev/null \
|| { \
    printf "\e[31mError: Unable to upload asset.\e[0m\n" \
    && exit 3; \
}

printf "\e[32mSuccess\e[0m\n"