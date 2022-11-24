#!/bin/bash

docker run -it --rm -v ${PWD}:/work -p 3000:3000 -w=/work node:19-bullseye /bin/bash