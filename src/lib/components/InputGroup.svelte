<script lang="ts">
  export let labelText = 'Not set';
  export let placeholder = 'Not set';
  export let value;
</script>

<div class="form-control">
  <!-- svelte-ignore a11y-label-has-associated-control -->
  <label class="input-group">
    <span class="w-24 flex justify-center">{labelText}</span>
    <input
      bind:value
      type="text"
      {placeholder}
      class="input input-bordered focus:border-secondary focus:outline-none focus:ring-0"
    />
  </label>
</div>
