<script setup>
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <header>
    <b-navbar class="p-3" toggleable="lg" type="light" variant="light">

      <b-navbar-brand href="#">ToDo App</b-navbar-brand>
      <b-navbar-toggle class="m-3" target="nav-collapse"></b-navbar-toggle>

      <b-collapse class="border-top" id="nav-collapse" is-nav>
      <b-navbar-nav>
        <b-nav-item to="/" exact>Home</b-nav-item>
        <b-nav-item to="/about">About</b-nav-item>
        <b-nav-item to="/todo">Todo List</b-nav-item>
      </b-navbar-nav>
    </b-collapse>
    </b-navbar>
  </header>
  <router-view/>
</template>
