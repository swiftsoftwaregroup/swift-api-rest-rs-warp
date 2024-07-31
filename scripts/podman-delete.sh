#!/usr/bin/env bash

# switch to parent directory
script_path=`dirname ${BASH_SOURCE[0]}`
pushd $script_path/..

# delete container
tag="latest"

image_name="swift-api-rs-warp:$tag"
container_name="swift-api-rs-warp"

echo "Delete old container ..."
podman rm -f $container_name

echo "Delete old image ..."
podman rmi $image_name

popd