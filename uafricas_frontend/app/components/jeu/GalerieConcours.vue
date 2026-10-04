<template>
  <div>
    <p v-if="!elements.length" class="rounded-[10px] border border-af-bordure bg-af-surface px-5 py-8 text-center text-[15px]/[1.5] text-af-corps">
      Aucune photo publiée pour l'instant.
    </p>
    <ul v-else class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4" role="list">
      <li v-for="p in elements" :key="p.id">
        <figure class="overflow-hidden rounded-[10px] border border-af-bordure bg-af-surface">
          <div class="relative">
            <img :src="urlMedia(p.media_url) ?? undefined" :alt="p.legende ?? 'Photo du concours'" class="aspect-square w-full object-cover" loading="lazy">
            <!-- Mode classé (après les résultats) : rang sur la photo. -->
            <span
              v-if="classee(p) && p.rang && !p.sous_seuil"
              class="absolute left-2 top-2 grid size-8 place-items-center rounded-full text-[14px] font-bold"
              :class="p.rang <= 3 ? 'bg-af-orange text-white' : 'bg-af-surface/90 text-af-encre'"
            >
              {{ p.rang }}
            </span>
          </div>
          <figcaption class="flex flex-col gap-1 p-2.5">
            <span v-if="p.legende" class="line-clamp-2 text-[13px]/[1.4] text-af-encre">{{ p.legende }}</span>
            <JeuSignalerParticipation v-if="concoursId && connecte" :concours-id="concoursId" :participation-id="p.id" />
            <template v-if="classee(p)">
              <span class="text-[12px]/[1.3] font-bold text-af-corps">{{ p.auteur.prenom }} {{ p.auteur.nom }}</span>
              <span v-if="p.taux != null" class="text-[12px]/[1.3] text-af-atone">
                {{ p.sous_seuil ? 'Hors classement' : `${p.taux} % de duels gagnés` }} · {{ p.duels }} duel{{ (p.duels ?? 0) > 1 ? 's' : '' }}
              </span>
            </template>
          </figcaption>
        </figure>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import type { ParticipationAnonymeAPI, ParticipationClasseeAPI } from '~/composables/useConcours'

/**
 * Galerie d'un concours (feature 014). Pendant l'appel et le vote, les photos
 * arrivent sans auteur et dans un ordre tiré par le serveur : ce composant
 * n'a rien à cacher, il n'a rien reçu. Après les résultats, elles arrivent
 * classées, auteurs révélés.
 */
defineProps<{
  elements: Array<ParticipationAnonymeAPI | ParticipationClasseeAPI>
  /** Permet le signalement (membres connectés). */
  concoursId?: string
}>()

const userStore = useUserStore()
const connecte = computed(() => !!userStore.accessToken)

const classee = (p: ParticipationAnonymeAPI | ParticipationClasseeAPI): p is ParticipationClasseeAPI =>
  'auteur' in p
</script>
