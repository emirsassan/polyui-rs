<template>
  <button id="poly-btn" class="button brand poly-btn">
    <slot></slot>
  </button>
</template>

<style>
.poly-btn {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  padding: 10px 32px;
  gap: 8px;
  
  position: relative;
  width: 244px;
  height: 48px;

  border-radius: 16px;
}
</style>