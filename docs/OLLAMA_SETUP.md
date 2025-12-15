# Ollama Setup - Local AI for PyPilot

Run AI models locally on your Mac with **100% privacy** and **no API costs**.

## Installation (1 minute)

```bash
# Install Ollama via Homebrew
brew install ollama

# Start Ollama service
brew services start ollama

# Or run manually
ollama serve
```

## Quick Start

1. **Open PyPilot Settings** (⚙️ icon in PyPilot panel)

2. **Select Provider**: Choose "Ollama (Local)"

3. **Select Model**: 
   - First time: PyPilot will show "Model not installed"
   - Click "Download Model" button
   - Choose recommended model (qwen2.5:7b)
   - Wait for download (~5 minutes for 4.7GB)

4. **Start using**: Type your question and PyPilot will use local AI!

## Recommended Models

### 🌟 qwen2.5:7b (BEST CHOICE)
- **Size**: 4.7 GB
- **Parameters**: 7 billion
- **Speed**: Very fast on M5
- **Strength**: Excellent tool calling, code generation
- **Use case**: General PyPilot tasks, MCP tools

```bash
ollama pull qwen2.5:7b
```

### ⚡ llama3.2:3b (FASTEST)
- **Size**: 2.0 GB  
- **Parameters**: 3 billion
- **Speed**: Ultra-fast
- **Strength**: Quick responses
- **Use case**: Simple queries, rapid prototyping

```bash
ollama pull llama3.2:3b
```

### 💪 qwen2.5:14b (HIGHEST QUALITY)
- **Size**: 9.0 GB
- **Parameters**: 14 billion
- **Speed**: Moderate
- **Strength**: Best reasoning
- **Use case**: Complex code refactoring
- **Requirement**: 16GB+ RAM

```bash
ollama pull qwen2.5:14b
```

## Manual Model Management

### List installed models
```bash
ollama list
```

### Download a model
```bash
ollama pull qwen2.5:7b
```

### Remove a model
```bash
ollama rm qwen2.5:7b
```

### Test a model
```bash
ollama run qwen2.5:7b "Write a hello world in VPy"
```

## Troubleshooting

### "Ollama is not running"
```bash
# Check if running
brew services info ollama

# Start service
brew services start ollama

# Or run manually in terminal
ollama serve
```

### "Download failed"
- Check internet connection
- Ensure enough disk space (~10GB free)
- Try manual download: `ollama pull qwen2.5:7b`

### "Model too slow"
- Use smaller model: llama3.2:3b
- Close other apps to free RAM
- Check Activity Monitor for CPU/RAM usage

### "Out of memory"
- Use 3B model instead of 7B/14B
- Close other applications
- Restart Mac to free memory

## Performance Comparison (on M5)

| Model | Size | Speed | Quality | RAM Usage |
|-------|------|-------|---------|-----------|
| llama3.2:3b | 2GB | ⚡⚡⚡ | ⭐⭐⭐ | ~4GB |
| qwen2.5:7b | 4.7GB | ⚡⚡ | ⭐⭐⭐⭐ | ~8GB |
| qwen2.5:14b | 9GB | ⚡ | ⭐⭐⭐⭐⭐ | ~12GB |

## Privacy & Benefits

✅ **100% Private**: All processing on your Mac  
✅ **No API Costs**: Unlimited usage  
✅ **No Internet Required**: Works offline (after download)  
✅ **Fast**: Optimized for Apple Silicon  
✅ **No Rate Limits**: Use as much as you want  

## Advanced Configuration

### Custom Base URL
If running Ollama on different port:

```
Provider: Ollama (Local)
Endpoint: http://localhost:12345
Model: qwen2.5:7b
```

### Using with Docker
```bash
docker run -d -p 11434:11434 --name ollama ollama/ollama
docker exec ollama ollama pull qwen2.5:7b
```

## Comparison: Cloud vs Local

| Feature | Cloud API | Ollama Local |
|---------|-----------|--------------|
| Cost | $0.15-$2 per 1M tokens | FREE |
| Privacy | Sent to cloud | 100% local |
| Speed | Network latency | Instant |
| Offline | ❌ Requires internet | ✅ Works offline |
| Rate Limits | Yes (strict) | ❌ Unlimited |
| Setup | API key needed | 1 min install |

## Next Steps

1. Install Ollama: `brew install ollama`
2. Start service: `brew services start ollama`
3. Open PyPilot → Settings → Select "Ollama (Local)"
4. Download qwen2.5:7b
5. Start coding with AI! 🚀

---

For more info: https://ollama.ai/
