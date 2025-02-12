<script lang="ts">
  type EditorTemplate = {
    key: string;
    description: string;
  };

  type Props = {
    value: string;
    onChange: (value: string) => void;
    onUserSave?: VoidFunction;
    templates: EditorTemplate[];
  };

  const { value, onChange, onUserSave, templates }: Props = $props();
</script>

<div class="template-split">
  <section class="editor">
    <textarea
      class="input"
      {value}
      onchange={(event) => {
        onChange(event.currentTarget.value);
      }}
    ></textarea>
  </section>

  <div class="hints">
    <h3>Templating</h3>

    <p>The following templates will be replaced if they are found</p>

    <ul class="templates">
      {#each templates as template}
        <li class="template">
          <span>$({template.key})</span> - {template.description}
        </li>
      {/each}
    </ul>
  </div>
</div>

<style>
  .editor {
    position: relative;
    overflow: hidden;
    height: 100%;
  }

  .template-split {
    display: flex;
    flex-direction: row;
    height: 100%;
  }

  .template-split .editor {
    flex: auto;
  }

  .hints {
    max-width: 14rem;
    padding: 1rem;
    height: 100%;
    overflow: auto;
  }

  .templates {
    list-style: none;
    display: flex;
    flex-flow: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .template {
    padding: 0.5rem;
    background-color: #1f1f1f;
  }

  .template > span {
    color: #e4b654;
  }

  .input {
    padding: 0.5rem;
    background-color: #000;
    border: 1px solid #666;
    color: #ccc;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    width: 100%;
    height: 100%;
    resize: none;
  }
</style>
