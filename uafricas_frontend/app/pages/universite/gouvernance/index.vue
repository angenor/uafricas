<script setup lang="ts">
import type { ContributionCitoyenne } from '~/types/gouvernance'

/**
 * Novagouv : porté sur le gabarit de la refonte.
 *
 * Les trois compteurs quittent le hero pour le rail, les trois cartes de
 * catégorie perdent leurs palettes bleu / jaune / rouge au profit des jetons
 * communs, et le hero cesse de charger son image depuis **images.unsplash.com** :
 * une page publique qui dépend d'un hôte tiers tombe avec lui, et l'image part
 * chez ce tiers l'adresse IP de chaque visiteur. `bonne_gouvernance.png` est
 * déjà dans le dépôt.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Novagouv : Gouvernance citoyenne | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Engager les citoyens africains et afro-descendants ainsi que la diaspora dans l'amélioration de la gouvernance politique, sociale et économique de l'Afrique.",
    }],
})

const { getStats, getContributions } = useGouvernance()

const chargement = ref(true)
const stats = ref({ total: 0, factcheck: 0, badhabits: 0, ideaforces: 0, totalLikes: 0 })
const dernieresContributions = ref<ContributionCitoyenne[]>([])

/** Les trois espaces citoyens. Le compteur est celui du même espace. */
const ESPACES = [
  {
    titre: 'Factcheck',
    description: 'Vérification des faits et lutte contre la désinformation.',
    icone: 'fa-solid fa-scale-balanced',
    vers: '/universite/gouvernance/factcheck',
    cle: 'factcheck' as const,
  },
  {
    titre: 'Ideaforces',
    description: "Propositions d'idées et forces positives pour l'Afrique.",
    icone: 'fa-solid fa-lightbulb',
    vers: '/universite/gouvernance/ideaforces',
    cle: 'ideaforces' as const,
  },
  {
    titre: 'BadGoodhabits',
    description: 'Signalement des mauvaises pratiques et des habitudes néfastes.',
    icone: 'fa-solid fa-triangle-exclamation',
    vers: '/universite/gouvernance/bad-good-habits',
    cle: 'badhabits' as const,
  }]

onMounted(async () => {
  try {
    const [statsData, contributionsData] = await Promise.all([
      getStats(),
      getContributions({ parPage: 6 })])
    stats.value = statsData
    dernieresContributions.value = contributionsData.contributions
  }
  catch (err) {
    console.error('Erreur chargement gouvernance:', err)
  }
  finally {
    chargement.value = false
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Novagouv"
        sous-titre="Contribuez à l'amélioration de notre société"
        image="/images/bonne_gouvernance.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Novagouv' }]"
      />
    </template>

    <div class="flex flex-col gap-8">
      <section class="flex flex-col gap-5">
        <div class="flex flex-col gap-2">
          <h2 class="text-[20px]/[1.4] font-bold text-af-chocolat">Les trois espaces citoyens</h2>
          <p class="text-[14px]/[1.4] text-af-corps">
            Engager les citoyens africains et afro-descendants ainsi que la diaspora dans
            l'amélioration de la gouvernance politique, sociale et économique de l'Afrique.
          </p>
        </div>

        <div class="grid gap-5 sm:grid-cols-2">
          <NuxtLink
            v-for="espace in ESPACES"
            :key="espace.titre"
            :to="espace.vers"
            class="group flex flex-col gap-2 rounded-[10px] border border-af-bordure bg-white p-6 transition hover:border-af-chocolat"
          >
            <font-awesome-icon :icon="espace.icone" class="size-8 text-af-chocolat transition group-hover:scale-110" />
            <h3 class="text-[17px]/[1.4] font-bold text-af-encre">{{ espace.titre }}</h3>
            <p class="text-[14px]/[1.4] text-af-corps">{{ espace.description }}</p>
            <span class="mt-auto flex items-center gap-2 pt-2 text-[14px]/[1.4] font-bold text-af-chocolat">
              Explorer
              <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition group-hover:translate-x-1" />
            </span>
          </NuxtLink>
        </div>
      </section>

      <section class="flex flex-col gap-5">
        <h2 class="text-[20px]/[1.4] font-bold text-af-chocolat">Dernières contributions</h2>

        <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
          <div v-for="n in 4" :key="n" class="h-48 animate-pulse rounded-[10px] bg-af-bordure" />
        </div>

        <div v-else-if="dernieresContributions.length" class="grid gap-5 sm:grid-cols-2">
          <UniversiteGouvernanceContributionCard
            v-for="contribution in dernieresContributions"
            :key="contribution.id"
            :contribution="contribution"
          />
        </div>

        <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
          <font-awesome-icon icon="fa-solid fa-comments" class="text-4xl text-af-atone-2" />
          <p class="mt-4 text-[16px]/[1.4] font-bold">Aucune contribution pour le moment</p>
          <p class="mt-2 text-[14px]/[1.4] text-af-corps">
            Les contributions des trois espaces apparaîtront ici dès leur publication.
          </p>
        </div>
      </section>
    </div>

    <template #rail>
      <AfricansPanneau titre="Contributions" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div
            v-for="(espace, i) in ESPACES"
            :key="espace.cle"
            class="flex items-baseline justify-between gap-4 py-3"
            :class="i === 0 ? 'pt-0' : 'border-t border-af-bordure'"
          >
            <dt class="text-[14px]/[1.4] font-bold">{{ espace.titre }}</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ stats[espace.cle] }}</dd>
          </div>
        </dl>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>
