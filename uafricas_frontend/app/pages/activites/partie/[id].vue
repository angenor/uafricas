<script setup lang="ts">
/**
 * L'écran de jeu, commun aux cadres libre, entraînement, défi et duel différé
 * (feature 013). Seuls changent la composition de la série et ce que le
 * résultat rapporte : le déroulé est le même.
 *
 *   suivante → épreuve + minuteur → répondre → correction → suivante → … → bilan
 *
 * Trois points tenus par le serveur, que cette page ne fait que refléter :
 *  - la bonne réponse n'arrive qu'avec la correction ;
 *  - le temps court depuis l'instant où le serveur a délivré l'épreuve ;
 *  - demander la suivante sans avoir répondu ENREGISTRE l'absence de réponse.
 *    Recharger la page ne permet donc pas de revoir une question : on reprend à
 *    la suivante.
 */
import {
  estRefusEntrainement,
  messageErreurJeu,
  type CorrectionAPI,
  type PartieAPI,
  type PresentationAPI,
} from '~/composables/useJeu'

definePageMeta({ layout: false, middleware: 'auth' })

useHead({ title: 'Partie | Activités | AfricanS' })

const route = useRoute()
const id = computed(() => String(route.params.id))

const { obtenirPartie, suivante, repondre, declarerInjouable, creerPartie, listerModules } = useJeu()

const partie = ref<PartieAPI | null>(null)
const presentation = ref<PresentationAPI | null>(null)
const correction = ref<CorrectionAPI | null>(null)
const choix = ref<number | null>(null)
const chargement = ref(true)
const envoi = ref(false)
const erreur = ref('')
/** L'épreuve précédente a été comptée sans réponse (rechargement, abandon). */
const repriseSignalee = ref(false)

const termine = computed(() => partie.value != null && partie.value.etat !== 'en_cours')

// Le fil d'Ariane affiche le LIBELLÉ du module, la partie n'en porte que le code.
// Chargé à part et sans attente : un échec laisse simplement le code affiché.
const libellesModules = ref<Record<string, string>>({})
const libelleModule = computed(() => {
  const code = partie.value?.module
  return code ? (libellesModules.value[code] ?? code) : null
})
const derniere = computed(
  () => presentation.value != null && presentation.value.rang >= presentation.value.sur,
)

const chargerBilan = async () => {
  partie.value = await obtenirPartie(id.value)
  presentation.value = null
  correction.value = null
}

/** Demande l'épreuve suivante ; bascule sur le bilan s'il n'y en a plus. */
const avancer = async () => {
  erreur.value = ''
  envoi.value = true
  try {
    const res = await suivante(id.value)
    if (!res || res.terminee) {
      await chargerBilan()
      return
    }
    repriseSignalee.value = res.precedente != null
    correction.value = null
    choix.value = null
    presentation.value = res
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Impossible de charger l\'épreuve suivante.')
  }
  finally {
    envoi.value = false
  }
}

/** Envoie la réponse. `cle = null` : le temps s'est écoulé. */
const envoyer = async (cle: number | null) => {
  if (!presentation.value || correction.value || envoi.value) return
  erreur.value = ''
  envoi.value = true
  choix.value = cle
  try {
    correction.value = await repondre(id.value, presentation.value.rang, cle)
  }
  catch (e) {
    choix.value = null
    erreur.value = messageErreurJeu(e, 'Votre réponse n\'a pas pu être enregistrée.')
  }
  finally {
    envoi.value = false
  }
}

const passerInjouable = async () => {
  if (!presentation.value || correction.value || envoi.value) return
  envoi.value = true
  try {
    correction.value = await declarerInjouable(id.value, presentation.value.rang)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e)
  }
  finally {
    envoi.value = false
  }
}

const rejouer = async () => {
  const module = partie.value?.module
  if (!module) return
  try {
    const nouvelle = await creerPartie(module)
    if (nouvelle) await navigateTo(`/activites/partie/${nouvelle.id}`)
  }
  catch (e) {
    // Plus d'épreuve neuve : c'est la page du module qui annonce l'entraînement.
    if (estRefusEntrainement(e)) await navigateTo(`/activites/${module}`)
    else erreur.value = messageErreurJeu(e, 'La partie n\'a pas pu être lancée.')
  }
}

const initialiser = async () => {
  chargement.value = true
  erreur.value = ''
  partie.value = null
  presentation.value = null
  correction.value = null
  try {
    partie.value = await obtenirPartie(id.value)
    if (partie.value?.etat === 'en_cours') await avancer()
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Cette partie est introuvable.')
  }
  finally {
    chargement.value = false
  }
}

onMounted(() => {
  initialiser()
  listerModules()
    .then((modules) => {
      libellesModules.value = Object.fromEntries(modules.map(m => [m.code, m.libelle]))
    })
    .catch(() => {})
})
// « Rejouer » mène à la même page avec un autre identifiant : le composant est
// réutilisé, il faut le réinitialiser à la main.
watch(id, initialiser)
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Activités', vers: '/activites' },
          ...(partie?.module && libelleModule ? [{ libelle: libelleModule, vers: `/activites/${partie.module}` }] : []),
          { libelle: 'Partie' },
        ]"
      />
    </template>

    <!-- `pb-24` : le dock de messagerie est ancré en bas de l'écran ; sans cette
         marge il recouvre « Épreuve suivante », dernier élément de la page. -->
    <div class="mx-auto flex w-full max-w-3xl flex-col gap-5 pb-24">
      <div v-if="chargement" class="h-72 animate-pulse rounded-[10px] bg-af-bordure" />

      <template v-else>
        <p
          v-if="erreur"
          class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
          role="alert"
        >
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
          {{ erreur }}
        </p>

        <JeuBilanPartie v-if="partie && termine" :partie="partie" @rejouer="rejouer" />

        <template v-else-if="presentation?.epreuve">
          <header class="flex flex-wrap items-center justify-between gap-3">
            <p class="text-[14px]/[1.4] font-bold text-af-encre">
              Épreuve {{ presentation.rang }} sur {{ presentation.sur }}
            </p>
            <AfricansEtiquette v-if="partie?.cadre === 'entrainement'">
              Entraînement, sans score
            </AfricansEtiquette>
            <AfricansEtiquette v-else-if="partie?.cadre === 'defi'" ton="vert">
              Défi
            </AfricansEtiquette>
          </header>

          <p v-if="repriseSignalee" class="text-[13px]/[1.4] text-af-atone">
            L'épreuve précédente est restée sans réponse : elle a été comptée ainsi.
          </p>

          <JeuMinuteur
            :expire-a="presentation.expire_a ?? presentation.maintenant"
            :maintenant="presentation.maintenant"
            :duree-s="partie?.temps_epreuve_s ?? 30"
            :arrete="correction != null"
            @expire="envoyer(null)"
          />

          <JeuCarteEpreuve
            :epreuve="presentation.epreuve"
            :choix="choix"
            :bonne-cle="correction?.bonne_cle ?? null"
            :verrouillee="envoi || correction != null"
            @choisir="envoyer"
            @injouable="passerInjouable"
          />

          <template v-if="correction">
            <JeuCorrection :correction="correction" />
            <div class="flex justify-end">
              <AfricansBouton
                :icone="derniere ? 'fa-solid fa-trophy' : 'fa-solid fa-chevron-right'"
                :desactive="envoi"
                @click="avancer"
              >
                {{ derniere ? 'Voir le bilan' : 'Épreuve suivante' }}
              </AfricansBouton>
            </div>
          </template>
        </template>
      </template>
    </div>
  </NuxtLayout>
</template>
