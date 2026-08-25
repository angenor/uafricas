<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="programme?.titre ?? 'Programme'"
        :sous-titre="programme ? `${programme.pays}${programme.ville ? ' · ' + programme.ville : ''}` : undefined"
        :image="programme?.couverture_url ?? null"
      >
        <template v-if="programme" #action>
          <span class="rounded-lg bg-af-vert px-4 py-2 text-[14px]/[1.4] font-bold text-white">
            {{ programme.interafricain ? 'Interafricain' : 'Hors Afrique vers Afrique' }}
          </span>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Sabbafrica', vers: '/echanges-sabbatiques' }, { libelle: programme?.titre ?? 'Programme' }]"
      />
    </template>

    <div v-if="!chargeFaite || (chargement && !programme)" class="flex flex-col gap-5">
      <div v-for="n in 3" :key="n" class="h-40 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <div v-else-if="!programme" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-plane" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Programme introuvable</p>
      <p class="mt-2 text-[14px]/[1.4] text-af-corps">Il a peut-être été retiré par son organisateur.</p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-arrow-left" vers="/echanges-sabbatiques">
        Retour aux programmes
      </AfricansBouton>
    </div>

    <div v-else class="flex flex-col gap-5">
      <div class="flex flex-wrap items-center gap-x-5 gap-y-2 rounded-[10px] border border-af-bordure bg-white p-5 text-[12px]/[1.4] text-af-atone">
        <span class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-location-dot" />
          {{ programme.pays }}<template v-if="programme.ville"> · {{ programme.ville }}</template>
        </span>
        <span class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-calendar-days" />
          {{ formatDateSabbatique(programme.date_debut) }}
        </span>
        <span class="flex items-center gap-1.5">
          <font-awesome-icon icon="fa-solid fa-clock" />
          {{ programme.duree_label }}
        </span>
        <AfricansEtiquette class="ml-auto" :ton="programme.statut === 'ouvert' ? 'vert' : 'gris'">
          {{ getStatutLabel(programme.statut) }}
        </AfricansEtiquette>
      </div>

      <!-- Le candidat retenu est affiché publiquement : c'est un choix de
           transparence de la feature, pas une fuite. -->
      <p
        v-if="programme.candidat_retenu"
        class="flex items-center gap-3 rounded-[10px] border border-af-vert/30 bg-af-vert/5 p-4 text-[14px]/[1.4] text-af-vert"
      >
        <font-awesome-icon icon="fa-solid fa-user-check" />
        <span>
          Candidat retenu :
          <strong class="font-bold">
            {{ programme.candidat_retenu.prenom ? `${programme.candidat_retenu.prenom} ` : '' }}{{ programme.candidat_retenu.nom }}
          </strong>
        </span>
      </p>

      <AfricansAccordeon titre="Description" icone="fa-solid fa-align-left" fond="blanc" par-defaut-ouvert>
        <!-- eslint-disable-next-line vue/no-v-html -->
        <div class="prose-af text-[14px]/[1.6] text-af-corps" v-html="sanitiserHtml(programme.description)" />
      </AfricansAccordeon>

      <AfricansAccordeon titre="Informations détaillées" icone="fa-solid fa-circle-info">
        <dl class="grid gap-5 sm:grid-cols-2">
          <div>
            <dt class="text-[12px]/[1.4] text-af-atone">Domaine d'intervention</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ programme.domaine || 'Non précisé' }}</dd>
          </div>
          <div>
            <dt class="text-[12px]/[1.4] text-af-atone">Durée</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ programme.duree_label }}</dd>
          </div>
          <div>
            <dt class="text-[12px]/[1.4] text-af-atone">Période</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-encre">
              Du {{ formatDateCourte(programme.date_debut) }}
              <template v-if="programme.date_fin"> au {{ formatDateCourte(programme.date_fin) }}</template>
            </dd>
          </div>
          <div>
            <dt class="text-[12px]/[1.4] text-af-atone">Lieu</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-encre">
              {{ programme.ville ? `${programme.ville}, ` : '' }}{{ programme.pays }}
            </dd>
          </div>

          <div v-if="programme.prise_en_charge.length" class="sm:col-span-2">
            <dt class="mb-2 text-[12px]/[1.4] text-af-atone">Prise en charge par l'organisation</dt>
            <dd class="flex flex-wrap gap-2">
              <AfricansEtiquette v-for="prise in programme.prise_en_charge" :key="prise" ton="vert">
                {{ getPriseLabel(prise) }}
              </AfricansEtiquette>
            </dd>
          </div>

          <div v-if="programme.prerequis" class="sm:col-span-2">
            <dt class="text-[12px]/[1.4] text-af-atone">Prérequis</dt>
            <dd class="text-[14px]/[1.4] whitespace-pre-line text-af-encre">{{ programme.prerequis }}</dd>
          </div>

          <div v-if="programme.langues_requises" class="sm:col-span-2">
            <dt class="text-[12px]/[1.4] text-af-atone">Langues requises</dt>
            <dd class="text-[14px]/[1.4] text-af-encre">{{ programme.langues_requises }}</dd>
          </div>
        </dl>
      </AfricansAccordeon>

      <AfricansAccordeon
        v-if="programme.type_organisation_label || programme.statut_legal || programme.user || programme.document_url"
        titre="L'organisation"
        icone="fa-solid fa-building"
      >
        <dl class="grid gap-5 sm:grid-cols-2">
          <div v-if="programme.type_organisation_label">
            <dt class="text-[12px]/[1.4] text-af-atone">Type d'organisation</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ programme.type_organisation_label }}</dd>
          </div>
          <div v-if="programme.statut_legal">
            <dt class="text-[12px]/[1.4] text-af-atone">Statut légal</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ programme.statut_legal }}</dd>
          </div>
          <div v-if="programme.user">
            <dt class="text-[12px]/[1.4] text-af-atone">Organisateur</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-encre">
              {{ programme.user.prenom ? `${programme.user.prenom} ` : '' }}{{ programme.user.nom }}
            </dd>
          </div>
          <div v-if="programme.document_url" class="sm:col-span-2">
            <dt class="text-[12px]/[1.4] text-af-atone">Document associé</dt>
            <dd>
              <a
                :href="programme.document_url"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
              >
                <font-awesome-icon icon="fa-solid fa-file-pdf" />
                Télécharger le document
              </a>
            </dd>
          </div>
        </dl>
      </AfricansAccordeon>

      <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-left" vers="/echanges-sabbatiques" class="self-start">
        Retour aux programmes
      </AfricansBouton>
    </div>

    <template #rail>
      <AfricansPanneau v-if="programme" titre="Candidature" icone="fa-solid fa-paper-plane">
        <div v-if="programme.est_organisateur" class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] text-af-corps">Vous êtes l'organisateur de ce programme.</p>
          <AfricansBouton pleine-largeur icone="fa-solid fa-users" @click="ouvrirCandidatures">
            Candidatures ({{ programme.nombre_candidatures }})
          </AfricansBouton>
        </div>

        <div v-else-if="programme.a_deja_candidate" class="flex items-start gap-3 text-[14px]/[1.4] text-af-vert">
          <font-awesome-icon icon="fa-solid fa-circle-check" class="mt-0.5" />
          Vous avez déjà candidaté à ce programme.
        </div>

        <div v-else-if="isAuthenticated" class="flex flex-col gap-3">
          <AfricansBouton
            pleine-largeur
            icone="fa-solid fa-paper-plane"
            :desactive="programme.statut !== 'ouvert'"
            @click="modaleCandidature = true"
          >
            {{ programme.statut === 'ouvert' ? 'Candidater' : getStatutLabel(programme.statut) }}
          </AfricansBouton>
          <p v-if="programme.statut !== 'ouvert'" class="text-[12px]/[1.4] text-af-atone">
            Les candidatures ne sont pas ouvertes pour ce programme.
          </p>
        </div>

        <div v-else class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] text-af-corps">Connectez-vous pour candidater à ce programme.</p>
          <AfricansBouton pleine-largeur icone="fa-solid fa-right-to-bracket" :vers="`/login?redirect=/echanges-sabbatiques/${programme.id}`">
            Se connecter
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <SabbatiqueCandidatureModal
      v-if="programme"
      :programme-id="programme.id"
      :open="modaleCandidature"
      @close="modaleCandidature = false"
      @success="onCandidatureSuccess"
    />

      <!-- Modale gestion des candidatures (organisateur) -->
      <Teleport to="body">
        <div
          v-if="modaleGestion"
          class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4 py-8 overflow-y-auto"
          @click.self="modaleGestion = false"
        >
          <div class="bg-white rounded-lg shadow-2xl max-w-2xl w-full my-auto">
            <div class="flex items-center justify-between px-6 py-4 border-b">
              <h2 class="text-lg font-bold text-custom-chocolat">
                Candidatures reçues
              </h2>
              <button type="button" class="text-gray-400 hover:text-gray-600" @click="modaleGestion = false">
                <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
              </button>
            </div>

            <div class="px-6 py-5 max-h-[70vh] overflow-y-auto">
              <p v-if="!candidatures.length" class="text-gray-500 text-sm text-center py-8">
                Aucune candidature pour le moment.
              </p>

              <div
                v-for="c in candidatures"
                :key="c.id"
                class="border rounded-lg p-4 mb-3"
                :class="c.est_retenu ? 'border-custom-green bg-custom-green/5' : 'border-gray-200'"
              >
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0">
                    <p class="font-semibold text-gray-900">
                      {{ c.nom_etat_civil || `${c.candidat.prenom || ''} ${c.candidat.nom}`.trim() }}
                    </p>
                    <p class="text-sm text-gray-600">{{ c.fonction_actuelle }}</p>
                    <p class="text-xs text-gray-400">{{ c.lieu_residence }}</p>
                    <div class="flex flex-wrap gap-2 mt-2 text-xs">
                      <span v-if="c.statut_emploi_label" class="px-2 py-0.5 bg-gray-100 rounded-full text-gray-600">
                        {{ c.statut_emploi_label }}
                      </span>
                      <span
                        class="px-2 py-0.5 rounded-full"
                        :class="c.repond_profil ? 'bg-green-100 text-green-700' : 'bg-amber-100 text-amber-700'"
                      >
                        {{ c.repond_profil ? 'Répond au profil' : 'Profil partiel' }}
                      </span>
                    </div>
                    <div class="flex flex-wrap gap-3 mt-2 text-sm">
                      <a v-if="c.cv_url" :href="c.cv_url" target="_blank" class="text-custom-green hover:underline">
                        <font-awesome-icon :icon="['fas', 'file-pdf']" class="mr-1" />CV
                      </a>
                      <a v-if="c.lien_expertise" :href="c.lien_expertise" target="_blank" class="text-custom-green hover:underline">
                        <font-awesome-icon :icon="['fas', 'link']" class="mr-1" />Compte expertise
                      </a>
                    </div>
                    <p v-if="c.lettre_motivation" class="text-sm text-gray-600 mt-2 italic">
                      « {{ c.lettre_motivation }} »
                    </p>
                  </div>
                  <div class="shrink-0">
                    <span v-if="c.est_retenu" class="inline-flex items-center gap-1 text-custom-green font-medium text-sm">
                      <font-awesome-icon :icon="['fas', 'award']" /> Retenu
                    </span>
                    <button
                      v-else
                      class="bg-custom-green text-white px-3 py-1.5 rounded-md text-sm hover:bg-custom-green/90 transition-colors disabled:opacity-50"
                      :disabled="selectionEnCours"
                      @click="retenir(c.id)"
                    >
                      Retenir
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Teleport>

      <!-- Toast succès candidature -->
      <Teleport to="body">
        <div
          v-if="toastSucces"
          class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-custom-green text-white px-5 py-3 rounded-lg shadow-lg flex items-center gap-2"
        >
          <font-awesome-icon :icon="['fas', 'circle-check']" />
          Votre candidature a bien été envoyée !
        </div>
      </Teleport>
      </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  useSabbatiques,
  formatDateSabbatique,
  formatDateCourte,
  PRISES_EN_CHARGE,
  type SabbatiqueDetailAPI,
  type CandidatureAPI,
} from '~/composables/useSabbatiques'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const {
  obtenirProgramme,
  chargement,
  listerCandidatures,
  selectionnerCandidat,
} = useSabbatiques()

const programme = ref<SabbatiqueDetailAPI | null>(null)
/**
 * Le chargement se fait dans `onMounted` : au rendu serveur, `chargement` est
 * encore faux et `programme` nul. Sans ce drapeau, la page affiche
 * « Programme introuvable » AVANT même d'avoir essayé : ce que voient les
 * robots d'indexation et l'utilisateur pendant un instant.
 */
const chargeFaite = ref(false)

const isAuthenticated = computed(() => userStore.isAuthenticated)

// État des modales et candidatures (organisateur)
const modaleCandidature = ref(false)
const modaleGestion = ref(false)
const toastSucces = ref(false)
const candidatures = ref<CandidatureAPI[]>([])
const selectionEnCours = ref(false)

const getStatutLabel = (statut: string) => {
  const labels: Record<string, string> = {
    ouvert: 'Inscriptions ouvertes',
    en_cours: 'En cours',
    termine: 'Terminé',
    complet: 'Complet'
  }
  return labels[statut] || statut
}

const getStatutClasses = (statut: string) => {
  const classes: Record<string, string> = {
    ouvert: 'bg-green-100 text-green-800',
    en_cours: 'bg-blue-100 text-blue-800',
    termine: 'bg-gray-100 text-gray-800',
    complet: 'bg-red-100 text-red-800'
  }
  return classes[statut] || 'bg-gray-100 text-gray-800'
}

const getPriseLabel = (value: string) => {
  const found = PRISES_EN_CHARGE.find(p => p.value === value)
  return found ? found.label : value
}

const rechargerProgramme = async () => {
  const id = route.params.id as string
  const result = await obtenirProgramme(id)
  if (result) programme.value = result
}

const onCandidatureSuccess = async () => {
  modaleCandidature.value = false
  toastSucces.value = true
  setTimeout(() => { toastSucces.value = false }, 4000)
  await rechargerProgramme()
}

const ouvrirCandidatures = async () => {
  if (!programme.value) return
  const liste = await listerCandidatures(programme.value.id)
  candidatures.value = liste || []
  modaleGestion.value = true
}

const retenir = async (candidatureId: string) => {
  if (!programme.value) return
  selectionEnCours.value = true
  const ok = await selectionnerCandidat(programme.value.id, candidatureId)
  selectionEnCours.value = false
  if (ok) {
    await Promise.all([rechargerProgramme(), ouvrirCandidatures()])
  }
}

onMounted(async () => {
  const id = route.params.id as string
  const result = await obtenirProgramme(id)
  programme.value = result
  chargeFaite.value = true

  if (programme.value) {
    useHead({
      title: `${programme.value.titre} - Échanges Sabbatiques - AfricanS`,
      meta: [
        {
          name: 'description',
          content: programme.value.description.substring(0, 160)
        }
      ]
    })
  }
})
</script>
