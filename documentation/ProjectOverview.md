# Project Overview

## Purpose:
The project is designed to analyze and interpret PDF files, converting their graphical content into structured data. It focuses on identifying shapes, lines, and points within the PDF and converting them into an SVG file format for further processing and visualization. This is achieved through a systematic process approach that ensures accuracy and efficiency in data extraction and conversion.

## Goals:
- **Shape Detection:** Identify and classify various shapes (lines, points, circles) from input PDF files.
- **Data Structuring:** Structure detected elements into nodes and pinpoints for clear visualization.
- **Visualization:** Convert processed data into an SVG file to visualize the detected shapes and their relationships.

## Key Functionalities:
- **PDF Processing:** Uses the pdfium-render library to render PDF pages into images.
- **Shape Detection:** Implements algorithms to detect shapes and lines within the rendered images.
- **Node and Pinpoint Creation:** Structures detected shapes into nodes and pinpoints, connecting them based on their relationships.
- **SVG Conversion:** Converts the structured data into SVG format for visualization.
- **Error Marking:** Marks unresolved parts of the image and saves them as a separate file for transparency.

## Use Case:
As a railway engineer at Siemens Mobility, I want to utilize a digital node-edge model of my railway network. Unfortunately, information gets delivered in old school vectorized PDF and TIFF formats. I’m far too lazy to manually engineer that huge variety of visual data.

Help me to recognize tracks, switches, signals, annotations, and other entities and to arrange them in a standardized model for my work. Bring us on the engineering fast track when we, for example, have to modernize interlockings or determine optimized and safe routes through the rail network. This is much more than just image recognition! The real challenge lies in the variability of how plans are drawn and how information is arranged.

My vision is to use the extracted topology data as input for further sophisticated railway solutions. To have a digital model that spans the entire engineering process and life cycle.
