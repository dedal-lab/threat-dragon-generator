#!/bin/bash

# Vérifier si les arguments sont fournis
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 path/to/your/document.pdf output_prefix"
    exit 1
fi

# Chemin vers le fichier PDF et préfixe pour les fichiers de sortie
pdf_path="$1"
output_prefix="$2"

# Vérifier si pdftoppm est installé
if ! command -v pdftoppm &> /dev/null
then
    echo "pdftoppm could not be found. Please install poppler-utils."
    exit 1
fi

# Vérifier si convert (ImageMagick) est installé
if ! command -v convert &> /dev/null
then
    echo "convert (ImageMagick) could not be found. Please install ImageMagick."
    exit 1
fi

# Convertir les pages PDF en images PNG
pdftoppm -png "$pdf_path" "$output_prefix"

# Rogner automatiquement les marges blanches de chaque image PNG générée
for img in ${output_prefix}-*.png; do
    convert "$img" -trim "$img"
done

echo "Conversion and trimming completed."