<script setup lang="ts">
import { Repository } from '@/types/api/repo'
defineOptions({
    name: 'HealthTab',
})

const { repo } = defineProps({
    repo: {
        type: Object as () => Repository,
        default: () => {},
    },
})
</script>

<template>
    <img
        :src="`https://api.star-history.com/chart?repos=${repo.owner_login}/${repo.name}&type=date&theme=dark&legend=top-left`"
        alt="health" />
</template>
