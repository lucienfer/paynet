<script lang="ts">
  import type { EventHandler } from "svelte/elements";
  import type { NodeData } from "../../types";
  import { addNode } from "../../commands";

  interface Props {
    nodes: NodeData[];
    onClose: () => void;
    onAddNode: (nodeData: NodeData) => void;
  }

  let { nodes, onClose, onAddNode }: Props = $props();

  let isLoading = $state(false);
  let errorMessage = $state("");

  const handleFormSubmit: EventHandler<SubmitEvent, HTMLFormElement> = async (
    event,
  ) => {
    isLoading = true;
    event.preventDefault();
    const form = event.target as HTMLFormElement;
    const formDataObject = new FormData(form);
    const nodeAddress = formDataObject.get("node-address");

    if (!!nodeAddress) {
      let nodeAddressString = nodeAddress.toString();
      addNode(nodeAddressString).then((newNodeData) => {
        if (!!newNodeData) {
          const nodeId = newNodeData[0];
          // Check if node with this ID already exists in the nodes array
          const nodeAlreadyListed = nodes.some((node) => node.id === nodeId);

          if (!nodeAlreadyListed) {
            onAddNode({
              id: nodeId,
              url: nodeAddressString,
              balances: newNodeData[1],
            });
          } else {
            console.log(`node with url ${nodeAddress} already declared`);
          }
        }
        onClose();
      });
    }
  };
</script>

<div class="modal-overlay">
  <div class="modal-content">
    <div class="modal-header">
      <h3>Add Node</h3>
      <button class="close-button" onclick={onClose}>✕</button>
    </div>

    <form onsubmit={handleFormSubmit}>
      <div class="form-group">
        <label for="node-address">Node's address</label>
        <input
          type="url"
          id="node-address"
          name="node-address"
          placeholder="https://example.com"
          required
        />
      </div>

      <button type="submit" class="submit-button" disabled={isLoading}>
        {isLoading ? "Adding node..." : "Add"}
      </button>
    </form>

    {#if errorMessage}
      <div class="error-message">
        {errorMessage}
      </div>
    {/if}
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background: white;
    border-radius: 12px;
    width: 90%;
    max-width: 400px;
    padding: 1.5rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.5rem;
    color: #333;
  }

  .close-button {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: #666;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    transition: background-color 0.2s;
  }

  .close-button:hover {
    background-color: #f0f0f0;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  .form-group label {
    display: block;
    font-size: 0.9rem;
    margin-bottom: 0.5rem;
    color: #333;
  }

  .form-group input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
  }

  .form-group input:focus {
    border-color: #1e88e5;
    outline: none;
    box-shadow: 0 0 0 2px rgba(30, 136, 229, 0.2);
  }

  .submit-button {
    padding: 0.8rem 2rem;
    background-color: #1e88e5;
    color: white;
    font-weight: 600;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    width: 100%;
    transition: background-color 0.2s;
  }

  .submit-button:hover:not(:disabled) {
    background-color: #1976d2;
  }

  .submit-button:disabled {
    background-color: #ccc;
    cursor: not-allowed;
  }

  .form-group input:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }

  .error-message {
    background-color: #fee2e2;
    color: #dc2626;
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.9rem;
    margin-top: 1rem;
    border: 1px solid #fecaca;
    text-align: center;
  }
</style>
