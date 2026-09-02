<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="formation?.titre ?? 'Formation'"
        :sous-titre="formation ? getTypeLabel(formation.type) : undefined"
        :image="formation?.couverture_url || '/images/education.png'"
      >
        <template v-if="formation" #action>
          <span class="rounded-lg bg-af-vert px-4 py-2 text-[14px]/[1.4] font-bold text-white">
            {{ getStatutLabel(formation.statut) }}
          </span>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Muniversa', vers: '/universite' },
          { libelle: 'Formations', vers: '/universite/formations' },
          { libelle: formation?.titre ?? 'Formation' },
        ]"
      />
    </template>

    <div v-if="chargement" class="flex flex-col gap-6">
      <div v-for="n in 2" :key="n" class="h-48 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="!formation" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-graduation-cap" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Formation introuvable</p>
      <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">
        Cette formation n'existe pas ou n'est plus proposée.
      </p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-arrow-left" vers="/universite/formations">
        Retour aux formations
      </AfricansBouton>
    </div>

    <!-- Les trois onglets deviennent des accordéons : dans une colonne de
         739 px, un menu vertical à gauche du contenu lui prenait un quart de
         sa largeur pour trois entrées. -->
    <div v-else class="flex flex-col gap-6">
      <AfricansAccordeon titre="Objectif" icone="fa-solid fa-bullseye" par-defaut-ouvert>
        <p v-if="formation.objectif" class="text-[14px]/[1.6] whitespace-pre-line text-af-corps">
          {{ formation.objectif }}
        </p>
        <p v-else class="text-[14px]/[1.4] text-af-atone italic">
          Les objectifs de cette formation seront précisés prochainement.
        </p>
      </AfricansAccordeon>

      <AfricansAccordeon titre="Présentation" icone="fa-solid fa-circle-info" par-defaut-ouvert>
        <p class="text-[14px]/[1.6] whitespace-pre-line text-af-corps">
          {{ formation.presentation || formation.description }}
        </p>
        <div
          v-if="formation.prerequis"
          class="mt-5 flex items-start gap-3 rounded-[10px] border border-af-bordure bg-af-fond p-4"
        >
          <font-awesome-icon icon="fa-solid fa-circle-check" class="mt-0.5 size-4 shrink-0 text-af-vert" />
          <div class="min-w-0">
            <p class="text-[14px]/[1.4] font-bold text-af-encre">Prérequis</p>
            <p class="mt-1 text-[14px]/[1.5] text-af-corps">{{ formation.prerequis }}</p>
          </div>
        </div>
      </AfricansAccordeon>

      <AfricansAccordeon titre="Intervenants" icone="fa-solid fa-chalkboard-user">
        <div class="flex items-start gap-4">
          <AfricansAvatar
            :nom="`${formation.formateur.prenom} ${formation.formateur.nom}`"
            :src="urlMedia(formation.formateur.photo_url)"
            :taille="64"
          />
          <div class="min-w-0">
            <p class="text-[14px]/[1.4] font-bold text-af-encre">
              {{ formation.formateur.prenom }} {{ formation.formateur.nom }}
            </p>
            <p class="text-[14px]/[1.4] text-af-vert">Responsable de la formation</p>
            <p class="text-[12px]/[1.4] text-af-atone">{{ formation.formateur.email }}</p>
          </div>
        </div>
        <!-- Deux cartes « Intervenant à confirmer » occupaient cette place.
             Elles annonçaient deux personnes de plus que rien ne garantit :
             une phrase dit la même chose sans dessiner de fantômes. -->
        <p class="mt-4 text-[14px]/[1.4] text-af-atone">
          D'autres intervenants pourront être annoncés d'ici au démarrage.
        </p>
      </AfricansAccordeon>

      <UniversiteInudaFormationCurriculum
        :formation-id="formation.id"
        :refresh-token="refreshContenu"
        @require-inscription="surRequireInscription"
      />
    </div>

    <template #rail>
      <template v-if="formation">
        <AfricansPanneau ref="inscriptionCard" titre="Inscription" icone="fa-solid fa-user-plus">
          <dl class="flex flex-col">
            <div
              v-for="(ligne, i) in ficheTechnique"
              :key="ligne.libelle"
              class="flex items-baseline justify-between gap-4 py-2.5"
              :class="i > 0 && 'border-t border-af-bordure'"
            >
              <dt class="text-[14px]/[1.4] text-af-atone">{{ ligne.libelle }}</dt>
              <dd class="text-right text-[14px]/[1.4] font-bold" :class="ligne.accent">{{ ligne.valeur }}</dd>
            </div>
          </dl>

          <a
            v-if="formation.lien_en_ligne"
            :href="formation.lien_en_ligne"
            target="_blank"
            rel="noopener noreferrer"
            class="mt-3 flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
          >
            <font-awesome-icon icon="fa-solid fa-link" />
            Accès en ligne
          </a>

          <div
            v-if="formation.est_inscrit"
            class="mt-5 flex items-center justify-center gap-2 rounded-lg bg-af-vert/10 px-4 py-3 text-[14px]/[1.4] font-bold text-af-vert"
          >
            <font-awesome-icon icon="fa-solid fa-circle-check" />
            Vous êtes inscrit(e)
          </div>
          <AfricansBouton
            v-else
            class="mt-5"
            pleine-largeur
            icone="fa-solid fa-user-plus"
            :desactive="!canInscribe"
            @click="sInscrire"
          >
            {{ actionLabel }}
          </AfricansBouton>

          <p v-if="erreurInscription" class="mt-3 text-[12px]/[1.4] text-af-live">{{ erreurInscription }}</p>
        </AfricansPanneau>
      </template>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
import {
  useFormations,
  type FormationDetailAPI,
  getTypeLabel,
  getTypeGradient,
  getStatutLabel,
  getStatutBadgeClass,
  getActionLabel,
  peutSInscrire,
  formatDateFormation,
  mapperFormatFrontend,
} from '~/composables/useFormations'

definePageMeta({ layout: false })

const route = useRoute()
const { chargement, obtenirFormation, inscrireFormation } = useFormations()

const formation = ref<FormationDetailAPI | null>(null)
const refreshContenu = ref(0)
const inscriptionCard = ref<HTMLElement | null>(null)
const erreurInscription = ref<string | null>(null)

const surRequireInscription = () => {
  inscriptionCard.value?.scrollIntoView({ behavior: 'smooth', block: 'center' })
}

/** La fiche technique du rail, dérivée d'un seul objet plutôt que de dix
 *  lignes de balisage se répétant à l'identique. */
const ficheTechnique = computed(() => {
  const f = formation.value
  if (!f) return []
  return [
    { libelle: 'Début', valeur: formatDateFormation(f.date_heure_debut), accent: '' },
    ...(f.date_heure_fin ? [{ libelle: 'Fin', valeur: formatDateFormation(f.date_heure_fin), accent: '' }] : []),
    { libelle: 'Langue', valeur: f.langue, accent: '' },
    { libelle: 'Format', valeur: mapperFormatFrontend(f.format), accent: '' },
    ...(f.pays ? [{ libelle: 'Territoire', valeur: f.pays, accent: '' }] : []),
    ...(f.ville ? [{ libelle: 'Ville', valeur: f.ville, accent: '' }] : []),
    ...(f.nombre_places ? [{ libelle: 'Places', valeur: `${f.nombre_inscrits}/${f.nombre_places}`, accent: '' }] : []),
    { libelle: 'Certifiante', valeur: f.est_certifiante ? 'Oui' : 'Non', accent: f.est_certifiante ? 'text-af-vert' : 'text-af-atone' },
    { libelle: 'Évaluation finale', valeur: f.a_evaluation ? 'Oui' : 'Non', accent: f.a_evaluation ? 'text-af-vert' : 'text-af-atone' },
  ]
})

const canInscribe = computed(() => {
  if (!formation.value) return false
  return peutSInscrire(formation.value)
})

const actionLabel = computed(() => {
  if (!formation.value) return ''
  return getActionLabel(formation.value)
})

const sInscrire = async () => {
  if (!formation.value) return
  erreurInscription.value = null
  const succes = await inscrireFormation(formation.value.id)
  if (succes) {
    const updated = await obtenirFormation(formation.value.id)
    if (updated) formation.value = updated
    refreshContenu.value++
  } else {
    // Un `alert()` bloque la page et sort du gabarit : le message vit
    // désormais à côté du bouton qui l'a produit.
    erreurInscription.value = "Erreur lors de l'inscription. Vérifiez que vous êtes connecté."
  }
}

onMounted(async () => {
  const id = route.params.id as string
  formation.value = await obtenirFormation(id)
  if (formation.value) {
    useHead({ title: `${formation.value.titre} | Muniversa` })
  }
})
</script>
