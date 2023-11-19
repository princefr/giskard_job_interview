<script setup lang="ts">
import IconUpload from '@/components/icons/IconUpload.vue';

defineProps<{
    handleFileChange: (event: Event) => void;
}>()
</script>

<!-- this is an icon from https://heroicons.com/ -->
<template>
            <div class="flex items-center justify-center w-full">
                <label for="file"
                    class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
                    <div class="flex flex-col items-center justify-center pt-2 pb-4">
                        <IconUpload />
                        <p class="mb-2 text-sm text-gray-500 dark:text-gray-400 p-4"><span class="font-semibold">Click to
                                upload</span>  the intercepted empire file</p>
                        <p class="text-xs text-gray-500 dark:text-gray-400">Upload your json file</p>
                    </div>
                    <input id="file" type="file" class="hidden" @change="handleFileChange" />
                </label>
            </div>
</template>

