<script>
  import Task from '../components/Task.vue'
  import { invoke } from "@tauri-apps/api/tauri";

  export default {
    data() {
      return {
        todos: {},
        errMsg: ""
    }
   },
    mounted (){
      this.fetchTodo()
    },
    methods: {
      async fetchTodo() {
        try {
          this.todos = await invoke("fetch_todo")
        } catch (error) {
          this.errMsg = error
        }
      }
    },
    components: {
      Task: Task,
    }
  }
</script>

<template>
  <main>
    <div v-if="errMsg" class="toast" role="alert" aria-live="assertive" aria-atomic="true">
      <div class="toast-header">
        <strong class="me-auto">Error</strong>
        <small>Just now</small>
        <button type="button" class="btn-close" data-bs-dismiss="toast" aria-label="Close"></button>
      </div>
      <div class="toast-body">
        {{ errMsg }}
      </div>
    </div>

    <ul>
      <li list-style-type: none v-for="task in todos">
        <Task :task="task"/>
      </li>
    </ul>
  </main>
</template>

<style scoped>
ul {
  list-style: none;
}
</style>
