<template>
  <article class="flex flex-col gap-2 rounded-[10px] border border-af-bordure bg-white p-4">
    <div class="flex items-start gap-2">
      <h4 class="line-clamp-1 flex-1 text-[14px]/[1.4] font-bold">{{ titre }}</h4>

      <span
        v-if="enDirect"
        class="flex shrink-0 items-center gap-1.5 text-[12px]/[1.4] font-bold text-af-live"
      >
        <span class="size-2 rounded-full bg-af-live" />
        En direct
      </span>

      <!-- Le cadenas n'est pas décoratif : il dit que l'entrée exige un code,
           ce qui explique pourquoi le bouton n'ouvre pas directement la salle. -->
      <font-awesome-icon
        icon="fa-solid fa-lock"
        class="mt-0.5 shrink-0 text-af-atone"
        title="Protégée par un code secret"
      />
    </div>

    <p v-if="description" class="line-clamp-2 text-[12px]/[1.4] text-af-corps">{{ description }}</p>

    <p class="flex items-center gap-2 text-[12px]/[1.4] text-af-atone">
      <AfricansAvatar :nom="auteurNom || 'Auteur inconnu'" :taille="24" />
      <span class="truncate">{{ estAuteur ? 'Vous' : (auteurNom || 'Auteur inconnu') }}</span>
    </p>

    <div class="mt-auto flex flex-col gap-2 pt-2">
      <AfricansBouton
        :variante="estAuteur ? 'primaire' : 'secondaire'"
        pleine-largeur
        :desactive="chargement"
        :tourne="chargement"
        :icone="chargement ? 'fa-solid fa-spinner' : (estAuteur ? 'fa-solid fa-door-open' : 'fa-solid fa-key')"
        @click="$emit(estAuteur ? 'ouvrir' : 'rejoindre')"
      >
        {{ chargement ? 'Connexion…' : (estAuteur ? 'Ouvrir ma salle' : 'Rejoindre') }}
      </AfricansBouton>

      <!-- Les deux actions d'auteur sont destructrices ou sensibles : elles
           restent en retrait, jamais au même poids que l'entrée dans la salle. -->
      <div v-if="estAuteur" class="flex items-center gap-4 text-[12px]/[1.4]">
        <button
          type="button"
          class="flex items-center gap-1.5 text-af-chocolat transition hover:opacity-70"
          @click="$emit('modifier-code')"
        >
          <font-awesome-icon icon="fa-solid fa-key" />
          Modifier le code
        </button>
        <button
          type="button"
          class="ml-auto flex items-center gap-1.5 text-af-live transition hover:opacity-70"
          @click="$emit('archiver')"
        >
          <font-awesome-icon icon="fa-solid fa-trash" />
          Supprimer
        </button>
      </div>
    </div>
  </article>
</template>

<script setup lang="ts">
/**
 * Cours privé d'une salle Afrolang. N'existe PAS dans la maquette : le canal
 * privé est une fonctionnalité livrée après elle. La carte reprend donc le
 * vocabulaire visuel de `CarteSalle` en plus compact, sans vignette : un cours
 * privé n'a pas d'image propre.
 */
defineProps<{
  titre: string
  description?: string | null
  auteurNom?: string | null
  estAuteur?: boolean
  enDirect?: boolean
  chargement?: boolean
}>()

defineEmits<{
  rejoindre: []
  ouvrir: []
  'modifier-code': []
  archiver: []
}>()
</script>
