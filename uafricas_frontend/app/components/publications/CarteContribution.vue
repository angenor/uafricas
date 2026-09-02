<template>
  <AfricansCartePublication
    :auteur="auteur"
    :categorie="STYLES[contribution.type].label"
    :titre="contribution.titre"
    :texte="contribution.description || undefined"
    :etiquettes="contribution.tags?.slice(0, 3).map(t => `#${t}`)"
    :actions="[]"
    :quand="quand"
  >
    <template #sous-media>
      <div class="flex flex-col gap-3 px-4 pb-1">
        <!-- Le préjugé et sa réfutation vont PAR PAIRE : montrer l'un sans
             l'autre ferait circuler le préjugé tout seul. -->
        <div v-if="contribution.type === 'factcheck' && contribution.factcheck" class="grid gap-3 sm:grid-cols-2">
          <div class="rounded border-l-4 border-af-live bg-af-live/[0.06] px-3 py-2">
            <p class="text-[12px]/[1.4] font-bold text-af-live">Préjugé</p>
            <p class="mt-1 line-clamp-2 text-[14px]/[1.4] text-af-corps">{{ contribution.factcheck.prejuge.titre }}</p>
          </div>
          <div class="rounded border-l-4 border-af-vert bg-af-vert/[0.06] px-3 py-2">
            <p class="text-[12px]/[1.4] font-bold text-af-vert">Réalité</p>
            <p class="mt-1 line-clamp-2 text-[14px]/[1.4] text-af-corps">{{ contribution.factcheck.contrePrejuge.titre }}</p>
          </div>
        </div>

        <div
          v-if="contribution.type === 'ideaforces' && contribution.proposition"
          class="rounded border-l-4 border-af-chocolat bg-af-chocolat/[0.07] px-3 py-2"
        >
          <p class="flex items-center gap-2 text-[12px]/[1.4] font-bold text-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-lightbulb" />
            Objectif
          </p>
          <p class="mt-1 line-clamp-2 text-[14px]/[1.4] text-af-corps">{{ contribution.proposition.objectif }}</p>
        </div>

        <div
          v-if="contribution.type === 'badhabits' && contribution.problematique"
          class="rounded border-l-4 border-af-live bg-af-live/[0.06] px-3 py-2"
        >
          <p class="flex items-center gap-2 text-[12px]/[1.4] font-bold text-af-live">
            <font-awesome-icon icon="fa-solid fa-tag" />
            {{ contribution.problematique.categorie }}
          </p>
          <p v-if="contribution.problematique.urgence" class="mt-1 text-[14px]/[1.4] text-af-corps">
            Urgence : {{ contribution.problematique.urgence }}
          </p>
        </div>

        <div class="flex flex-wrap gap-2">
          <AfricansEtiquette v-if="contribution.verified" ton="vert">Vérifié</AfricansEtiquette>
          <AfricansEtiquette v-if="contribution.problematique?.gravite">
            Gravité : {{ contribution.problematique.gravite }}
          </AfricansEtiquette>
        </div>
      </div>
    </template>

    <!-- Compteurs en LECTURE SEULE : cette page n'a pas d'endpoint pour réagir
         à une contribution, des boutons cliquables n'y mèneraient nulle part. -->
    <template #actions>
      <span class="flex items-center gap-2 text-af-atone">
        <font-awesome-icon icon="fa-solid fa-eye" />
        {{ contribution.stats.vues }}
      </span>
      <span class="flex items-center gap-2 text-af-atone">
        <font-awesome-icon icon="fa-solid fa-thumbs-up" />
        {{ contribution.stats.likes }}
      </span>
      <span class="flex items-center gap-2 text-af-atone">
        <font-awesome-icon icon="fa-solid fa-comment" />
        {{ contribution.stats.commentaires }}
      </span>
      <NuxtLink
        :to="STYLES[contribution.type].vers"
        class="flex items-center gap-2 text-af-chocolat transition hover:opacity-70"
      >
        <font-awesome-icon icon="fa-solid fa-arrow-right" />
        {{ STYLES[contribution.type].label }}
      </NuxtLink>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import type { ContributionCitoyenne } from '~/types/gouvernance'

/**
 * Contribution citoyenne (FactCheck, IdeaForces, BadGoodhabits) dans le fil.
 * Elle n'a pas de page de détail propre : le lien renvoie vers le module, et
 * les compteurs sont affichés sans être actionnables.
 */
const props = defineProps<{ contribution: ContributionCitoyenne }>()

const STYLES: Record<string, { label: string, vers: string }> = {
  factcheck: { label: 'FactCheck', vers: '/universite/gouvernance/factcheck' },
  ideaforces: { label: 'IdeaForces', vers: '/universite/gouvernance/ideaforces' },
  badhabits: { label: 'BadGoodhabits', vers: '/universite/gouvernance/bad-good-habits' },
}

const auteur = computed(() => ({
  nom: `${props.contribution.auteur.prenom} ${props.contribution.auteur.nom}`.trim() || 'Anonyme',
  lieu: props.contribution.localisation?.pays || undefined,
}))

const quand = computed(() => {
  const d = props.contribution.dateCreation instanceof Date
    ? props.contribution.dateCreation
    : new Date(props.contribution.dateCreation)
  return new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(d)
})
</script>
