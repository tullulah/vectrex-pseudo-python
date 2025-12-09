// MCP Test Script - Quick verification of MCP server functionality
// Run this in browser console after IDE loads to test MCP

async function testMCPServer() {
  console.log('=== MCP Server Test ===\n');
  
  if (!window.mcp) {
    console.error('❌ window.mcp not available - MCP API not exposed');
    return;
  }
  
  console.log('✓ MCP API available');
  
  // Test 1: List tools
  console.log('\n1. Testing tools/list...');
  try {
    const toolsResponse = await window.mcp.request({
      jsonrpc: '2.0',
      id: 1,
      method: 'tools/list',
      params: {},
    });
    console.log('✓ tools/list response:', toolsResponse);
  } catch (error) {
    console.error('❌ tools/list failed:', error);
  }
  
  // Test 2: List documents
  console.log('\n2. Testing editor/list_documents...');
  try {
    const docsResponse = await window.mcp.request({
      jsonrpc: '2.0',
      id: 2,
      method: 'editor/list_documents',
      params: {},
    });
    console.log('✓ editor/list_documents response:', docsResponse);
    if (docsResponse.result && docsResponse.result.documents) {
      console.log(`  Found ${docsResponse.result.documents.length} open documents`);
    }
  } catch (error) {
    console.error('❌ editor/list_documents failed:', error);
  }
  
  // Test 3: Get diagnostics
  console.log('\n3. Testing editor/get_diagnostics...');
  try {
    const diagsResponse = await window.mcp.request({
      jsonrpc: '2.0',
      id: 3,
      method: 'editor/get_diagnostics',
      params: {},
    });
    console.log('✓ editor/get_diagnostics response:', diagsResponse);
    if (diagsResponse.result && diagsResponse.result.diagnostics) {
      console.log(`  Found ${diagsResponse.result.diagnostics.length} diagnostics`);
    }
  } catch (error) {
    console.error('❌ editor/get_diagnostics failed:', error);
  }
  
  // Test 4: Get emulator state
  console.log('\n4. Testing emulator/get_state...');
  try {
    const emuResponse = await window.mcp.request({
      jsonrpc: '2.0',
      id: 4,
      method: 'emulator/get_state',
      params: {},
    });
    console.log('✓ emulator/get_state response:', emuResponse);
  } catch (error) {
    console.error('❌ emulator/get_state failed:', error);
  }
  
  // Test 5: Get project structure
  console.log('\n5. Testing project/get_structure...');
  try {
    const projectResponse = await window.mcp.request({
      jsonrpc: '2.0',
      id: 5,
      method: 'project/get_structure',
      params: {},
    });
    console.log('✓ project/get_structure response:', projectResponse);
  } catch (error) {
    console.error('❌ project/get_structure failed:', error);
  }
  
  console.log('\n=== Test Complete ===');
}

// Export for manual testing
if (typeof window !== 'undefined') {
  (window as any).testMCPServer = testMCPServer;
  console.log('MCP test script loaded. Run testMCPServer() to test.');
}
