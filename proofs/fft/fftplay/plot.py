# People use regression with stock prices because it's continuous data.
# This is different from classificiation which is more about grouping labels.

# Supervised features labels

import datetime
import os
import math

import pandas
import matplotlib.pyplot as plot
from matplotlib import style


def get_block_at_seconds(data, seconds, sample_rate, frame_size):
    start = seconds * sample_rate
    end = start + frame_size
    end = start + sample_rate
    block = data[start:end][["magnitude"]]
    return block


def load_spec():
    data = pandas.read_csv("specs.csv")

    return {
        "sample-rate": data["sample-rate"][0],
        "channels": data["channels"][0],
        "bits": data["bits"][0],
        "sample-count": data["sample-count"][0],
        "frame-size": data["frame-size"][0],
    }


def main():
    GRAPH_COUNT = 10

    data = pandas.read_csv("spectrum.csv")
    spec = load_spec()
    print(data.head())

    for i in range(0, spec["sample-count"], spec["frame-size"]):
        end = int((spec["frame-size"] / 2) + i)
        block = data[i:end][["magnitude"]]
        plot.plot(block)

        GRAPH_COUNT -= 1
        if GRAPH_COUNT == 0:
            break

    plot.xlabel("Frequency")
    plot.ylabel("Magnitude")
    plot.show()


if __name__ == "__main__":
    main()
