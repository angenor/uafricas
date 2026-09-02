<script setup lang="ts">
import { useFacultes, type Faculte } from '~/composables/useFacultes'

/**
 * Facultés partenaires de Muniversa, portées sur le gabarit de la refonte.
 *
 * La page lisait `~/mocks/inuda/facultes` : des facultés INVENTÉES, avec leurs
 * photos hébergées chez unsplash, pendant que le back-office en gérait de
 * vraies et que `useFacultes` savait déjà les servir. Les deux ne se sont
 * jamais rencontrés — ajouter une faculté en administration ne changeait rien
 * ici, et les visiteurs voyaient un catalogue qui n'existait pas.
 *
 * Le filtrage passe côté SERVEUR : l'endpoint porte `recherche`, `domaine`,
 * `type_ecole` et `ouvertes`, et renvoie au passage la liste des domaines
 * réellement déclarés — plus besoin de la déduire du jeu de données affiché,
 * ce qui masquait tout domaine absent de la première page.
 */
definePageMeta({ layout: false })

useHead({ title: 'Facultés partenaires | Muniversa' })

const { listerFacultes, loading, erreur } = useFacultes()

const facultes = ref<Faculte[]>([])
const domaines = ref<string[]>([])
const total = ref(0)

const recherche = ref('')
const domaineSelectionne = ref('')
const typeEcole = ref('')
const seulementOuvertes = ref(false)

const AUTRES_INUDA = [
  { libelle: 'Formations', to: '/universite/formations', icone: 'fa-solid fa-graduation-cap' },
  { libelle: 'Mon espace', to: '/universite/mon-espace', icone: 'fa-solid fa-user-graduate' },
  { libelle: 'Africalive', to: '/evenements/liste', icone: 'fa-solid fa-calendar-days' },
]

const faculteSelectionnee = ref<Faculte | null>(null)

const aucunFiltreActif = computed(
  () => !recherche.value && !domaineSelectionne.value && !typeEcole.value && !seulementOuvertes.value,
)

const charger = async () => {
  try {
    const data = await listerFacultes({
      recherche: recherche.value || undefined,
      domaine: domaineSelectionne.value || undefined,
      typeEcole: typeEcole.value || undefined,
      ouvertes: seulementOuvertes.value || undefined,
      parPage: 24,
    })
    facultes.value = data.facultes
    total.value = data.total
    // Les domaines viennent du SERVEUR, sur l'ensemble du catalogue : les
    // déduire des seules facultés affichées ferait disparaître un domaine
    // dès qu'un filtre l'écarte, et le rendrait impossible à resélectionner.
    if (data.domaines?.length) domaines.value = data.domaines
  }
  catch {
    // `erreur` est déjà posée par le composable.
  }
}

const reinitialiser = () => {
  recherche.value = ''
  domaineSelectionne.value = ''
  typeEcole.value = ''
  seulementOuvertes.value = false
}

let minuterie: ReturnType<typeof setTimeout> | null = null
watch(recherche, () => {
  if (minuterie) clearTimeout(minuterie)
  minuterie = setTimeout(charger, 400)
})
watch([domaineSelectionne, typeEcole, seulementOuvertes], charger)

onMounted(charger)

const voirDetail = (faculte: Faculte) => navigateTo(`/universite/facultes/${faculte.id}`)
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Nos facultés partenaires"
        sous-titre="Les facultés qui collaborent avec Muniversa pour votre réussite académique"
        image="/images/education.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Mindshiftlab', vers: '/mindshiftlab' },
          { libelle: 'Muniversa', vers: '/universite' },
          { libelle: 'Facultés' },
        ]"
      >
        <template #centre>
          <p class="text-base font-bold text-af-encre">
            {{ total }} faculté{{ total > 1 ? 's' : '' }}
          </p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <div
        v-if="erreur"
        class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="shrink-0" />
        <span class="min-w-0 flex-1">{{ erreur }}</span>
        <button type="button" class="shrink-0 font-bold underline" @click="charger">Réessayer</button>
      </div>

      <div v-if="loading" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-80 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="facultes.length > 0" class="grid gap-5 sm:grid-cols-2">
        <article
          v-for="faculte in facultes"
          :key="faculte.id"
          class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat"
        >
          <div class="relative aspect-[16/9] overflow-hidden bg-af-fond">
            <img
              v-if="faculte.imageCouverture"
              :src="faculte.imageCouverture"
              :alt="faculte.titre"
              class="size-full object-cover transition-transform duration-500 group-hover:scale-105"
              loading="lazy"
            />
            <!-- Pas de repli sur un fichier : une faculté sans couverture
                 affiche un pictogramme, qui ne peut pas manquer. -->
            <div v-else class="grid size-full place-items-center">
              <font-awesome-icon icon="fa-solid fa-building-columns" class="text-4xl text-af-atone-2" />
            </div>
            <span
              v-if="faculte.accepteNouveauxInscrits"
              class="absolute top-3 right-3 rounded bg-af-vert px-3 py-1 text-[12px]/[1.4] font-bold text-white"
            >
              Inscriptions ouvertes
            </span>
          </div>

          <div class="flex flex-1 flex-col gap-3 p-5">
            <div>
              <h2 class="text-[17px]/[1.4] font-bold text-af-encre">{{ faculte.titre }}</h2>
              <p class="text-[12px]/[1.4] text-af-atone">{{ faculte.acronyme }}</p>
            </div>

            <p class="line-clamp-2 text-[14px]/[1.5] text-af-corps">{{ faculte.description }}</p>

            <p v-if="faculte.ecolePartenaire" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
              <font-awesome-icon icon="fa-solid fa-location-dot" />
              {{ faculte.ecolePartenaire.nom }}, {{ faculte.ecolePartenaire.ville }}, {{ faculte.ecolePartenaire.pays }}
            </p>

            <div v-if="faculte.domainesEtudes?.length" class="flex flex-wrap gap-2">
              <AfricansEtiquette v-for="d in faculte.domainesEtudes.slice(0, 3)" :key="d">{{ d }}</AfricansEtiquette>
              <AfricansEtiquette v-if="faculte.domainesEtudes.length > 3">
                +{{ faculte.domainesEtudes.length - 3 }}
              </AfricansEtiquette>
            </div>

            <div class="mt-auto flex flex-wrap items-center gap-3 pt-2">
              <AfricansBouton variante="secondaire" icone="fa-solid fa-circle-info" @click="voirDetail(faculte)">
                En savoir plus
              </AfricansBouton>
              <AfricansBouton
                :desactive="!faculte.accepteNouveauxInscrits"
                icone="fa-solid fa-paper-plane"
                @click="faculteSelectionnee = faculte"
              >
                {{ faculte.accepteNouveauxInscrits ? 'Manifester mon intérêt' : 'Inscriptions fermées' }}
              </AfricansBouton>
            </div>
          </div>
        </article>
      </div>

      <!-- Deux vides distincts : « rien ne correspond » n'est pas « aucun
           partenariat », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-building-columns" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ aucunFiltreActif ? 'Aucune faculté partenaire pour le moment' : 'Aucune faculté ne correspond à vos critères' }}
        </p>
        <AfricansBouton
          v-if="!aucunFiltreActif"
          class="mt-6"
          variante="secondaire"
          icone="fa-solid fa-rotate-left"
          @click="reinitialiser"
        >
          Réinitialiser les filtres
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansRecherche v-model="recherche" placeholder="Nom, acronyme, domaine…" />

      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiser">
        <div class="flex flex-col gap-4">
          <AfricansChamp v-model="domaineSelectionne" libelle="Domaine d'études" type="select">
            <option value="">Tous les domaines</option>
            <option v-for="d in domaines" :key="d" :value="d">{{ d }}</option>
          </AfricansChamp>

          <AfricansChamp v-model="typeEcole" libelle="Type d'établissement" type="select">
            <option value="">Tous les types</option>
            <option value="publique">Public</option>
            <option value="privee">Privé</option>
          </AfricansChamp>

          <label class="flex items-center gap-3 text-[14px]/[1.4] text-af-corps">
            <input v-model="seulementOuvertes" type="checkbox" class="size-4 accent-af-chocolat" />
            Inscriptions ouvertes uniquement
          </label>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Aussi dans Muniversa" icone="fa-solid fa-graduation-cap">
        <ul class="flex flex-col gap-1">
          <li v-for="lien in AUTRES_INUDA" :key="lien.to">
            <NuxtLink
              :to="lien.to"
              class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-[14px]/[1.4] font-bold text-af-corps transition hover:bg-af-chocolat/[0.07] hover:text-af-chocolat"
            >
              <font-awesome-icon :icon="lien.icone" class="size-5 shrink-0" />
              {{ lien.libelle }}
            </NuxtLink>
          </li>
        </ul>
      </AfricansPanneau>
    </template>

    <AfricansModale
      :model-value="faculteSelectionnee !== null"
      titre="Manifester mon intérêt"
      icone="fa-solid fa-paper-plane"
      @update:model-value="faculteSelectionnee = null"
    >
      <p v-if="faculteSelectionnee" class="text-[14px]/[1.5] text-af-corps">
        Vous manifestez votre intérêt pour
        <strong class="font-bold text-af-encre">{{ faculteSelectionnee.titre }}</strong>.
      </p>
      <!-- Dit ce qui est : aucun endpoint ne reçoit encore cette manifestation.
           Un formulaire qui n'envoie rien serait pire qu'une annonce franche. -->
      <p class="mt-3 text-[14px]/[1.4] text-af-atone">
        La candidature en ligne n'est pas encore ouverte. En attendant, contactez directement
        l'établissement partenaire.
      </p>

      <template #actions>
        <AfricansBouton @click="faculteSelectionnee = null">J'ai compris</AfricansBouton>
      </template>
    </AfricansModale>
  </NuxtLayout>
</template>
