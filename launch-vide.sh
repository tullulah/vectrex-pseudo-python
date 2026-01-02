#!/bin/bash

# Script para lanzar Vide con VIDE_HOME configurado
# Uso: ./launch-vide.sh

# Obtener directorio donde está este script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Configurar VIDE_HOME al directorio Vide.osx
export VIDE_HOME="$SCRIPT_DIR/vide/Vide.osx"

echo "Configurando VIDE_HOME=$VIDE_HOME"
echo "Lanzando Vide..."

# Ejecutar Vide.app
open "$VIDE_HOME/Vide.app"
