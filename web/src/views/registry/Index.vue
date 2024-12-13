<script setup>
import { ref } from 'vue'
import { getVersion } from '@/api'
import { useRouter } from 'vue-router'
const router = useRouter()

getVersion()
  .then(res => {
    version.value = res.data.version
  })
  .finally(() => {
    if (!version.value) {
      router.push('/registry/error')
    } else {
      router.push('/registry/create')
    }
  })

const version = ref('')

</script>

<template>
  <router-view />
</template>
