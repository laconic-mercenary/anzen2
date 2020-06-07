package net.anzen2;

import org.apache.log4j.Logger;

import com.github.sarxos.webcam.Webcam;

public class Main {

    private final static Logger LOGGER = Logger.getLogger(Main.class);

    public static void main(String[] args) {
        LOGGER.info(">>>>>>");
        Webcam webcam = Webcam.getDefault();
        LOGGER.info(webcam.getViewSizes());
    }
}