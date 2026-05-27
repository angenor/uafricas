<script setup lang="ts">
// Nœud-jonction : matérialise l'union d'un couple parental.
// Les branches des deux parents y convergent, et celles des enfants en repartent,
// ce qui rend visible que les enfants sont issus du couple (et non d'un seul parent).
import { Handle, Position } from '@vue-flow/core'
</script>

<template>
  <div class="h-2.5 w-2.5 rounded-full bg-[#8a5a2b] shadow ring-2 ring-white/70">
    <Handle id="union-haut" type="target" :position="Position.Top" class="!h-1 !w-1 !border-0 !bg-[#8a5a2b] !opacity-0" />
    <Handle id="union-bas" type="source" :position="Position.Bottom" class="!h-1 !w-1 !border-0 !bg-[#8a5a2b] !opacity-0" />
  </div>
</template>
