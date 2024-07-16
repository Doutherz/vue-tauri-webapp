<template>
  <div class="task-item p-0">
    <div class="bg-dark p-2 m-0 text-white rounded-end">{{ task.id }}</div>
    <input class="form-check-input m-3 p-3" type="checkbox" v-model="task.is_complete">
    <div class="task-details p-3 border-start bg-white">
      <div class="task-title">{{ task.title }}</div>
      <div class="task-content">{{ task.task }}</div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  task: {
    type: {},
    required: true
  }
})
</script>

<style scoped>

.task-checkbox {
  display: inline-block;
  position: relative;
  width: 20px;
  height: 20px;
  margin: 10px;
  padding: 10px;
  cursor: pointer;
}

.task-checkbox input {
  position: absolute;
  opacity: 0;
  cursor: pointer;
}

.checkbox-custom {
  position: absolute;
  top: 0;
  left: 0;
  width: 20px;
  height: 20px;
  background-color: transparent;
  border: 2px solid #007bff; /* Checkbox border color */
  border-radius: 4px;
}

.task-checkbox input:checked + .checkbox-custom {
  background-color: #007bff; /* Fill color when checked */
}


.task-item {
  display: flex;
  align-items: center;
  border: 1px solid #ddd;
  padding: 10px;
  margin: 10px 0;
  border-radius: 5px;
  background-color: #f9f9f9;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.task-id {
  font-size: 0.8em;
  color: #999;
  margin-right: 20px;
}

.task-details {
  flex-grow: 1;
}

.task-title {
  font-size: 1.2em;
  font-weight: bold;
  margin: 5px 0;
}

.task-content {
  font-size: 1em;
  color: #333;
}
</style>
