#!/bin/bash
# Test script for MCP stdio server with proper Content-Length headers

MESSAGE='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}'
LENGTH=${#MESSAGE}

# Send request with Content-Length header
(
  echo "Content-Length: $LENGTH"
  echo ""
  echo -n "$MESSAGE"
) | node ide/mcp-server/server.js 2>&1 | head -n 50
