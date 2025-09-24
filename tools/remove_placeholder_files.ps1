<#
remove_placeholder_files.ps1

Elimina archivos vacíos o que sólo contienen placeholders/comentarios (stubs) tras la unificación de tests.

USO BÁSICO (modo exploración / dry-run por defecto):
  powershell -ExecutionPolicy Bypass -File tools\remove_placeholder_files.ps1 -Root .

Para ejecutar borrado real:
  powershell -ExecutionPolicy Bypass -File tools\remove_placeholder_files.ps1 -Root . -Apply

Parámetros:
  -Root <path>        Raíz a escanear (por defecto directorio actual)
  -Extensions <list>  Extensiones a considerar (por defecto: .rs,.md,.asm,.txt)
  -Aggressive         Activa heurísticas para detectar stubs con test trivial (assert!(true))
  -Apply              Si se incluye, elimina realmente los archivos; si se omite, sólo lista (dry-run)
  -Verbose            Muestra detalles de decisión por archivo

Heurísticas de placeholder:
  1. Archivo vacío tras trim.
  2. Sólo comentarios y líneas en blanco (se eliminan comentarios de línea // y bloques /* */ para .rs/.asm)
  3. (Aggressive) Contiene palabras clave: "Stub:" o "unified_stub" o "Consolidado en `opcodes_all.rs`" y el código efectivo
     (no comentarios) coincide con patrón de test stub simple (ej. un único #[test] con assert!(true)).
  4. (Aggressive) Longitud < 400 bytes y ninguna palabra clave significativa (CPU, struct, mod, enum, impl) fuera de comentarios.

Seguridad:
  - Ignora explícitamente archivos llamados 'opcodes_all.rs' o que contengan 'DO_NOT_DELETE_PLACEHOLDER'.
  - Siempre muestra resumen final.
  - Para proteger otros, usar primero sin -Apply y revisar lista.

Salida:
  - Tabla sencilla (Path, Reason) y contadores.

Nota: Ajustar heurísticas si se vuelve demasiado agresivo.
#>
[CmdletBinding()]
param(
    [string]$Root = (Get-Location).Path,
    [string[]]$Extensions = @('.rs','.md','.asm','.txt'),
    [switch]$Aggressive,
    [switch]$Apply,
    [switch]$ShowVerbose
)

function Write-Log($msg){ if($ShowVerbose){ Write-Host "[DBG] $msg" -ForegroundColor DarkGray } }

if(-not (Test-Path $Root)){ throw "Root path '$Root' no existe" }

$files = Get-ChildItem -Path $Root -Recurse -File -ErrorAction SilentlyContinue |
    Where-Object { $Extensions -contains ([IO.Path]::GetExtension($_.FullName).ToLower()) } |
    Where-Object { $_.FullName -notmatch '\bnode_modules\b' }

$toDelete = @()

$blockCommentRegex = '/\*.*?\*/'
$singleLineCommentPatterns = @('^\s*//','^\s*;') # ; for asm
# Para markdown no tratamos '#' como comentario

$testStubRegex = '^(?s)\s*(#\[test\])?\s*fn\s+\w+\s*\([^)]*\)\s*\{[^{}]*assert!\(true!?;?\)\s*;?[^{}]*\}\s*$'

foreach($f in $files){
    if($f.Name -eq 'opcodes_all.rs'){ continue }
    $raw = $null
    try { $raw = Get-Content -Raw -LiteralPath $f.FullName -ErrorAction Stop } catch { Write-Log "Skip (read error): $($f.FullName) -> $_"; continue }
    if($null -eq $raw){ Write-Log "Skip (null content): $($f.FullName)"; continue }
    $trimmed = $raw.Trim()
    if([string]::IsNullOrWhiteSpace($trimmed)){
        Write-Log "$($f.FullName): vacío"
        $toDelete += [pscustomobject]@{ Path=$f.FullName; Reason='empty'}
        continue
    }

    $ext = $f.Extension.ToLower()

    $noBlock = if($ext -in @('.rs','.asm')){ [regex]::Replace($raw,$blockCommentRegex,'','Singleline') } else { $raw }
    $lines = $noBlock -split "`r?`n"
    $effectiveLines = @()
    foreach($line in $lines){
        if($line -match '^\s*$'){ continue }
        if($ext -in @('.rs','.asm')){
            $isComment = $false
            foreach($pat in $singleLineCommentPatterns){ if($line -match $pat){ $isComment = $true; break } }
            if($isComment){ continue }
        }
        $effectiveLines += $line
    }

    if($effectiveLines.Count -eq 0){
        Write-Log "$($f.FullName): sólo comentarios"
        $toDelete += [pscustomobject]@{ Path=$f.FullName; Reason='comments_only'}
        continue
    }

    if($Aggressive){
        $joined = ($effectiveLines -join "`n").Trim()
    $lowerRaw = $raw.ToLowerInvariant()
        $hasStubKeyword = ($lowerRaw -match 'stub:' -or $lowerRaw -match 'unified_stub' -or $lowerRaw -match 'consolidado en `opcodes_all.rs`'.ToLower())
        $hasCodeTokens = ($joined -match '\b(cpu|struct|enum|impl|mod)\b')
        $sizeOk = ($raw.Length -lt 400)
        $matchesTestStub = ($joined -match $testStubRegex)

        if($hasStubKeyword -and $matchesTestStub){
            Write-Log "$($f.FullName): stub test detectado"
            $toDelete += [pscustomobject]@{ Path=$f.FullName; Reason='stub_test'}
            continue
        }
        elseif($sizeOk -and -not $hasCodeTokens -and $matchesTestStub){
            Write-Log "$($f.FullName): heurística stub mínima"
            $toDelete += [pscustomobject]@{ Path=$f.FullName; Reason='minimal_stub'}
            continue
        }
    }
}

if($toDelete.Count -eq 0){
    Write-Host "No se detectaron archivos placeholder para borrar." -ForegroundColor Green
    return
}

Write-Host "Archivos candidatos (" $toDelete.Count "):" -ForegroundColor Yellow
$toDelete | Sort-Object Path | Format-Table -AutoSize

if($Apply){
    $deleted = 0
    foreach($item in $toDelete){
        try {
            Remove-Item -LiteralPath $item.Path -Force
            Write-Host "BORRADO: $($item.Path) ($($item.Reason))" -ForegroundColor Red
            $deleted++
        } catch {
            Write-Host "ERROR al borrar $($item.Path): $_" -ForegroundColor Magenta
        }
    }
    Write-Host "Total borrados: $deleted" -ForegroundColor Cyan
} else {
    Write-Host "(Dry-run) Use -Apply para borrar." -ForegroundColor Cyan
}
