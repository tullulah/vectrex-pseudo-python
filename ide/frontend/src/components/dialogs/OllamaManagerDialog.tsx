import React, { useState, useEffect } from 'react';
import { OllamaProvider, OllamaModelInfo } from '../../services/providers/OllamaProvider';

interface OllamaManagerDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onModelSelected: (modelName: string) => void;
  currentModel?: string;
}

interface InstalledModel {
  name: string;
  size: number;
  modified_at: string;
}

export const OllamaManagerDialog: React.FC<OllamaManagerDialogProps> = ({
  isOpen,
  onClose,
  onModelSelected,
  currentModel
}) => {
  const [isOllamaInstalled, setIsOllamaInstalled] = useState(false);
  const [installedModels, setInstalledModels] = useState<InstalledModel[]>([]);
  const [recommendedModels, setRecommendedModels] = useState<OllamaModelInfo[]>([]);
  const [downloadProgress, setDownloadProgress] = useState<{ [key: string]: number }>({});
  const [downloadingModel, setDownloadingModel] = useState<string | null>(null);
  const [isInstallingOllama, setIsInstallingOllama] = useState(false);
  const [ollamaInstallProgress, setOllamaInstallProgress] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (isOpen) {
      checkOllamaStatus();
    }
  }, [isOpen]);

  const checkOllamaStatus = async () => {
    setLoading(true);
    const provider = new OllamaProvider({ model: 'qwen2.5:7b' });
    
    // Check if Ollama is installed and running
    const isRunning = await provider.checkOllamaRunning();
    setIsOllamaInstalled(isRunning);

    if (isRunning) {
      // Get installed models
      const models = await fetchInstalledModels();
      setInstalledModels(models);
      
      // Get recommended models
      const recommended = await provider.getRecommendedModels();
      setRecommendedModels(recommended);
    }
    
    setLoading(false);
  };

  const fetchInstalledModels = async (): Promise<InstalledModel[]> => {
    try {
      const response = await fetch('http://localhost:11434/api/tags');
      if (!response.ok) return [];
      const data = await response.json();
      return data.models || [];
    } catch {
      return [];
    }
  };

  const installOllama = async () => {
    setIsInstallingOllama(true);
    setOllamaInstallProgress('Checking Homebrew...');

    try {
      // Check if Homebrew is installed
      const hasBrewResponse = await (window as any).electron.runCommand('which brew');
      
      if (!hasBrewResponse.success || !hasBrewResponse.output.trim()) {
        setOllamaInstallProgress('Installing Homebrew first...');
        const brewInstall = await (window as any).electron.runCommand(
          '/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
        );
        if (!brewInstall.success) {
          throw new Error('Failed to install Homebrew');
        }
      }

      setOllamaInstallProgress('Installing Ollama via Homebrew...');
      const installResponse = await (window as any).electron.runCommand('brew install ollama');
      
      if (!installResponse.success) {
        throw new Error('Failed to install Ollama');
      }

      setOllamaInstallProgress('Starting Ollama service...');
      await (window as any).electron.runCommand('brew services start ollama');

      // Wait a bit for service to start
      setOllamaInstallProgress('Waiting for service to start...');
      await new Promise(resolve => setTimeout(resolve, 3000));

      setOllamaInstallProgress('Ollama installed successfully! ✅');
      await checkOllamaStatus();
      
      setTimeout(() => {
        setIsInstallingOllama(false);
        setOllamaInstallProgress('');
      }, 2000);
    } catch (error) {
      console.error('Ollama installation failed:', error);
      setOllamaInstallProgress(`Installation failed: ${error}`);
      setTimeout(() => {
        setIsInstallingOllama(false);
        setOllamaInstallProgress('');
      }, 5000);
    }
  };

  const downloadModel = async (modelName: string) => {
    setDownloadingModel(modelName);
    setDownloadProgress({ ...downloadProgress, [modelName]: 0 });

    try {
      const provider = new OllamaProvider({ model: modelName });
      
      await provider.pullModel(modelName, (progress) => {
        setDownloadProgress(prev => ({ ...prev, [modelName]: progress }));
      });

      // Refresh installed models list
      const models = await fetchInstalledModels();
      setInstalledModels(models);
      
      setDownloadingModel(null);
      delete downloadProgress[modelName];
      
      // Auto-select the newly downloaded model
      onModelSelected(modelName);
    } catch (error) {
      console.error('Model download failed:', error);
      setDownloadingModel(null);
      alert(`Failed to download ${modelName}: ${error}`);
    }
  };

  const selectModel = (modelName: string) => {
    onModelSelected(modelName);
    onClose();
  };

  const formatSize = (bytes: number): string => {
    const gb = bytes / (1024 * 1024 * 1024);
    return `${gb.toFixed(1)} GB`;
  };

  const isModelInstalled = (modelName: string): boolean => {
    return installedModels.some(m => m.name === modelName);
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg shadow-xl max-w-3xl w-full max-h-[80vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="px-6 py-4 border-b border-gray-700 flex items-center justify-between">
          <h2 className="text-xl font-bold text-white flex items-center gap-2">
            🏠 Ollama Model Manager
          </h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            ✕
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {loading ? (
            <div className="text-center text-gray-400 py-8">
              Loading Ollama status...
            </div>
          ) : !isOllamaInstalled ? (
            // Ollama not installed
            <div className="text-center py-8">
              <div className="text-6xl mb-4">🏠</div>
              <h3 className="text-xl font-bold text-white mb-2">Ollama Not Installed</h3>
              <p className="text-gray-400 mb-6">
                Ollama is required to run local AI models. Would you like to install it?
              </p>
              
              {isInstallingOllama ? (
                <div className="max-w-md mx-auto">
                  <div className="bg-gray-700 rounded-lg p-4">
                    <div className="text-sm text-white mb-2">{ollamaInstallProgress}</div>
                    <div className="w-full bg-gray-600 rounded-full h-2">
                      <div className="bg-blue-500 h-2 rounded-full animate-pulse" style={{ width: '100%' }} />
                    </div>
                  </div>
                </div>
              ) : (
                <button
                  onClick={installOllama}
                  className="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
                >
                  Install Ollama via Homebrew
                </button>
              )}
            </div>
          ) : (
            // Ollama installed - show models
            <div className="space-y-6">
              {/* Installed Models */}
              {installedModels.length > 0 && (
                <div>
                  <h3 className="text-lg font-bold text-white mb-3">📦 Installed Models</h3>
                  <div className="space-y-2">
                    {installedModels.map(model => (
                      <div
                        key={model.name}
                        className={`p-4 rounded-lg border ${
                          currentModel === model.name
                            ? 'border-blue-500 bg-blue-500 bg-opacity-10'
                            : 'border-gray-700 bg-gray-750'
                        }`}
                      >
                        <div className="flex items-center justify-between">
                          <div className="flex-1">
                            <div className="font-medium text-white">{model.name}</div>
                            <div className="text-sm text-gray-400">
                              Size: {formatSize(model.size)} • Modified: {new Date(model.modified_at).toLocaleDateString()}
                            </div>
                          </div>
                          <button
                            onClick={() => selectModel(model.name)}
                            className={`px-4 py-2 rounded-lg font-medium transition-colors ${
                              currentModel === model.name
                                ? 'bg-blue-600 text-white'
                                : 'bg-gray-700 hover:bg-gray-600 text-white'
                            }`}
                          >
                            {currentModel === model.name ? '✓ Active' : 'Select'}
                          </button>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Recommended Models */}
              <div>
                <h3 className="text-lg font-bold text-white mb-3">⭐ Recommended Models</h3>
                <div className="space-y-2">
                  {recommendedModels.map(model => {
                    const installed = isModelInstalled(model.name);
                    const isDownloading = downloadingModel === model.name;
                    const progress = downloadProgress[model.name] || 0;

                    return (
                      <div
                        key={model.name}
                        className="p-4 rounded-lg border border-gray-700 bg-gray-750"
                      >
                        <div className="flex items-start justify-between gap-4">
                          <div className="flex-1">
                            <div className="font-medium text-white flex items-center gap-2">
                              {model.displayName}
                              {model.recommended && <span className="text-xs bg-blue-600 px-2 py-0.5 rounded">RECOMMENDED</span>}
                            </div>
                            <div className="text-sm text-gray-400 mt-1">{model.description}</div>
                            <div className="text-xs text-gray-500 mt-1">
                              Size: {model.size} • Parameters: {model.parameters}
                            </div>
                          </div>
                          
                          {installed ? (
                            <button
                              onClick={() => selectModel(model.name)}
                              className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg font-medium transition-colors whitespace-nowrap"
                            >
                              Select
                            </button>
                          ) : isDownloading ? (
                            <div className="w-32">
                              <div className="text-xs text-gray-400 mb-1">{progress}%</div>
                              <div className="w-full bg-gray-600 rounded-full h-2">
                                <div
                                  className="bg-blue-500 h-2 rounded-full transition-all"
                                  style={{ width: `${progress}%` }}
                                />
                              </div>
                            </div>
                          ) : (
                            <button
                              onClick={() => downloadModel(model.name)}
                              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors whitespace-nowrap"
                            >
                              Download
                            </button>
                          )}
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-gray-700 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg font-medium transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
