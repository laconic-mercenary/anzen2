### OVERVIEW

#### Algorithm
1. Determine stable environment image:
    - determine what the scene looks like with no motion
    - Calculate the average of the pixel values
2. Compare incoming frames with the average
    - if the threshold is broken
        - if the next N frames also break the threshold
            - trigger 